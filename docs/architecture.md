# VRCJournal - システムアーキテクチャ

## 1. 技術スタック

### フロントエンド
- **フレームワーク**: Vue 3 (Composition API, `<script setup>`)
- **ビルドツール**: Vite
- **言語**: TypeScript
- **多言語対応**: Vue I18n
- **日付処理**: Day.js (localizedFormat, updateLocale)
- **UIライブラリ**: Lucide Vue (アイコン)
- **スタイル**: CSS Variables (カスタムデザインシステム)

### バックエンド
- **フレームワーク**: Tauri 2.x
- **言語**: Rust
- **データベース**: SQLite 3
- **ファイル監視**: notify クレート
- **OSプラグイン**: tauri-plugin-os (システムロケール検出)

### プラットフォーム
- **対応OS**: Windows

## 2. アーキテクチャ原則

### 2.1 イベント駆動アーキテクチャ
VRCJournalは**完全なイベント駆動アーキテクチャ**を採用します。

**基本原則:**
1. **ログの変更は即座にイベントを発火する**
2. **フロントエンドは常にイベントを監視して動的に更新する**
3. **フロントエンドからバックエンドへのデータ要求は最小限にする**
   - 初回ページロード時のみデータ取得
   - ページ切り替え時のみデータ取得
   - それ以外はイベント経由で自動更新

### 2.2 データフロー
```
VRChatログファイル
  ↓ (notify - ファイル変更検知)
LogWatcher
  ↓ (新しい行を読み取り)
LogParser
  ↓ (LogEventに変換)
EventProcessor
  ↓ (データベースに保存 + ProcessedEventに変換)
データベース (SQLite)
  ↓ (Tauriイベント発行)
フロントエンド (Vue)
  ↓ (リアクティブに表示更新)
UI更新完了
```

## 3. コアコンポーネント

### 3.1 LogWatcher (バックエンド)
**責務**: VRChatログファイルの監視とリアルタイム読み取り

**処理フロー:**
```rust
// 起動時
1. VRChatログディレクトリを特定
   - %USERPROFILE%\AppData\LocalLow\VRChat\VRChat\
2. データベースから処理済みファイル位置を取得
3. 全ログファイルを初回読み込み (過去ログも処理)
4. ファイル監視を開始 (notify)

// 実行時
1. ファイル変更イベントを受信
2. 新しい行のみを読み取り
3. LogParserに渡す
4. ファイル位置をデータベースに保存
```

**重要な実装:**
- ファイル位置 (byte offset) をデータベースに永続化
- アプリ再起動後も続きから読み取り
- 複数のログファイルを同時監視

### 3.2 LogParser (バックエンド)
**責務**: ログ行を構造化されたLogEventに変換

**処理するログイベント:**
```rust
pub enum LogEvent {
    UserAuthenticated {
        timestamp: DateTime<Utc>,
        display_name: String,
        user_id: String,
    },
    JoiningWorld {
        timestamp: DateTime<Utc>,
        world_id: String,
        instance_id: String,
        world_name: String,
    },
    EnteringRoom {
        timestamp: DateTime<Utc>,
        world_name: String,
    },
    PlayerJoined {
        timestamp: DateTime<Utc>,
        display_name: String,
        user_id: String,
    },
    DestroyingPlayer {
        timestamp: DateTime<Utc>,
        display_name: String,
    },
    AvatarChanged {
        timestamp: DateTime<Utc>,
        display_name: String,
        avatar_name: String,
    },
    ScreenshotTaken {
        timestamp: DateTime<Utc>,
        file_path: String,
    },
}
```

**パース処理:**
- 正規表現によるログ行マッチング
- タイムスタンプ解析
- ワールドID・インスタンスID抽出
- プレイヤー情報抽出

### 3.3 EventProcessor (バックエンド)
**責務**: LogEventを処理してデータベースに保存し、フロントエンド通知イベントを生成

**内部状態:**
```rust
pub struct EventProcessor {
    current_local_player_id: Option<i64>,     // 現在ログイン中のアカウント
    current_session_id: Option<i64>,          // 現在のセッション
    player_ids: HashMap<String, i64>,         // user_id -> player_id マッピング
    pending_avatars: HashMap<String, (i64, DateTime<Utc>)>, // 保留中のアバター情報
}
```

**イベント処理フロー:**
```rust
LogEvent → process_event() → データベース保存 → Option<ProcessedEvent>
```

**発行するProcessedEvent:**
```rust
pub enum ProcessedEvent {
    LocalPlayerUpdated,                        // アカウント追加・更新
    SessionCreated { session_id: i64 },        // 新セッション作成
    SessionEnded { session_id: i64, ended_at: String }, // セッション終了
    PlayerJoined { session_id: i64 },          // プレイヤー参加
    PlayerLeft { session_id: i64 },            // プレイヤー退出
}
```

**イベント処理ルール:**

| LogEvent | データベース操作 | ProcessedEvent | 備考 |
|----------|------------------|----------------|------|
| UserAuthenticated | ローカルプレイヤー作成/更新 | LocalPlayerUpdated | アカウント切り替え検知 |
| JoiningWorld | セッション作成 | SessionCreated | 前セッションは自動的にinterrupted |
| EnteringRoom | セッションのworld_name更新 | なし | メタデータ補完のみ |
| PlayerJoined | プレイヤー作成/更新 + session_players追加 | PlayerJoined | プレイヤー数更新 |
| DestroyingPlayer (自分) | セッション終了 + 全員退出処理 | SessionEnded | セッション完了 |
| DestroyingPlayer (他) | session_playersのleft_at更新 | PlayerLeft | プレイヤー数更新 |
| AvatarChanged | アバター記録 + session_avatars更新 | なし | 将来的にアバター履歴機能で使用 |
| ScreenshotTaken | スクリーンショット記録 | なし | セッション詳細で表示 |

### 3.4 Database (バックエンド)
**責務**: データの永続化とクエリ

**スキーマ設計:**
- `players` - プレイヤー情報 (is_local=1がローカルプレイヤー)
- `sessions` - セッション情報
- `session_players` - セッション参加プレイヤー (joined_at, left_at)
- `avatars` - アバター情報
- `session_avatars` - セッション中のアバター使用履歴
- `screenshots` - スクリーンショット
- `player_name_history` - プレイヤー名前変更履歴
- `log_files` - 処理済みログファイル位置

**重要なインデックス:**
- `sessions.player_id` - アカウント別セッション取得
- `session_players.session_id` - セッション内プレイヤー取得
- `screenshots.session_id` - セッション内スクリーンショット取得

### 3.5 Frontend (Vue)
**責務**: UIレンダリングとユーザーインタラクション

**イベントリスナー設定:**
```typescript
// App.vue (起動時に設定)
onMounted(async () => {
  // 初回データ読み込み
  await loadUsers();
  await loadSessions();

  // イベントリスナー登録
  unlistenFn = await listen<ProcessedEvent>("log-event", (event) => {
    const processedEvent = event.payload;

    switch (processedEvent.type) {
      case "LocalPlayerUpdated":
        loadUsers();  // アカウントリスト再取得
        break;

      case "SessionCreated":
        loadSessions();  // セッションリスト再取得
        break;

      case "SessionEnded":
        // セッションのステータスをローカルで更新
        updateSessionStatus(processedEvent.session_id, 'completed');
        break;

      case "PlayerJoined":
      case "PlayerLeft":
        // 該当セッションのプレイヤー数を再取得
        refreshSessionPlayerCount(processedEvent.session_id);
        break;
    }
  });
});
```

**データ取得ルール:**
- **初回ロード時**: `get_local_users()`, `get_sessions()` を呼び出し
- **イベント受信時**: 必要に応じて再取得
- **ページ切り替え時**: `get_sessions()` を必要なら再取得
- **それ以外**: バックエンドからのプッシュを待つ

## 4. データフロー詳細

### 4.1 アプリ起動シーケンス
```
1. Tauriアプリ起動
   ↓
2. データベース初期化 (migrate)
   ↓
3. EventProcessor初期化
   - データベースから最新のローカルプレイヤー取得
   - 進行中のセッション復元
   - セッション参加プレイヤーのマッピング復元
   ↓
4. LogWatcher起動
   - 処理済みログファイル位置を取得
   - 全ログファイルを初回読み込み
   - ファイル監視開始
   ↓
5. フロントエンド初期化
   - 初回データ取得 (get_local_users, get_sessions)
   - イベントリスナー登録 (log-event)
   ↓
6. 待機状態 (ログ監視中)
```

### 4.2 ログ変更時のシーケンス
```
1. VRChatがログファイルに書き込み
   ↓
2. notify がファイル変更を検知
   ↓
3. LogWatcher が新しい行を読み取り
   ↓
4. LogParser が LogEvent に変換
   ↓
5. EventProcessor が処理
   - データベース保存
   - ProcessedEvent 生成 (通知が必要な場合のみ)
   ↓
6. Tauri が "log-event" を emit
   ↓
7. フロントエンドのイベントリスナーが受信
   ↓
8. イベントタイプに応じてUIを更新
   - LocalPlayerUpdated → アカウントリスト再取得
   - SessionCreated → セッションリスト再取得
   - SessionEnded → セッションステータス更新
   - PlayerJoined/Left → プレイヤー数更新
```

### 4.3 セッション作成フロー
```
VRChatログ: "[Behaviour] Joining wrld_xxx:12345~region(us)~..."
   ↓
LogParser: LogEvent::JoiningWorld
   ↓
EventProcessor:
   1. 前のセッションを interrupted にする (もしあれば)
   2. 新しいセッションをデータベースに作成
   3. current_session_id を更新
   4. player_ids, pending_avatars をクリア
   5. ProcessedEvent::SessionCreated を返す
   ↓
Tauri emit: "log-event" { type: "SessionCreated", session_id: 123 }
   ↓
フロントエンド:
   1. loadSessions() を呼び出し
   2. セッションリストを再レンダリング
   3. 新しいセッションがタイムラインのトップに表示される
```

### 4.4 プレイヤー参加フロー
```
VRChatログ: "[Behaviour] OnPlayerJoined Username123"
              "[Behaviour] Initialized PlayerAPI "Username123" is local"
   ↓
LogParser: LogEvent::PlayerJoined { display_name, user_id }
   ↓
EventProcessor:
   1. players テーブルにプレイヤーを作成/更新
   2. player_name_history に名前履歴を追加
   3. session_players に参加記録を追加
   4. player_ids マッピングに追加
   5. pending_avatars があれば記録
   6. ProcessedEvent::PlayerJoined を返す
   ↓
Tauri emit: "log-event" { type: "PlayerJoined", session_id: 123 }
   ↓
フロントエンド:
   1. 該当セッションのプレイヤー数を再取得 (軽量)
   2. SessionCard のプレイヤー数バッジを更新
```

## 5. 非イベント駆動部分 (例外)

以下の機能はユーザーアクション時に**Command呼び出し**を行います:

### 5.1 初回データロード
- `get_local_users()` - アプリ起動時
- `get_sessions()` - アプリ起動時、アカウント切り替え時

### 5.2 詳細情報の取得
- `get_session_by_id(session_id)` - セッション詳細ビュー表示時
- `get_session_players(session_id)` - セッション詳細ビュー表示時
- `get_session_screenshots(session_id)` - セッション詳細ビュー表示時

### 5.3 ユーザーアクション
- `open_invite_url(world_id, instance_id)` - ワールドを開くボタンクリック時
- `open_screenshot_directory(file_path)` - フォルダを開くボタンクリック時
- `open_user_page(user_id)` - プレイヤーページを開くボタンクリック時

## 6. 状態管理

### 6.1 バックエンド状態 (EventProcessor)
```rust
// アプリケーション全体で1つのEventProcessorインスタンス
pub struct EventProcessor {
    current_local_player_id: Option<i64>,
    current_session_id: Option<i64>,
    player_ids: HashMap<String, i64>,
    pending_avatars: HashMap<String, (i64, DateTime<Utc>)>,
}
```

**状態遷移:**
- `UserAuthenticated` → current_local_player_id 更新
- `JoiningWorld` → current_session_id 更新、マップクリア
- `PlayerJoined` → player_ids に追加
- `DestroyingPlayer (自分)` → current_session_id クリア、マップクリア

### 6.2 フロントエンド状態 (Vue Reactivity)
```typescript
// App.vue
const localUsers = ref<LocalUser[]>([]);     // アカウントリスト
const sessions = ref<Session[]>([]);         // セッションリスト
const selectedUserId = ref<number | null>(null);  // 選択中のアカウント
```

**状態更新:**
- イベント受信時に必要なデータのみ再取得
- リアクティブなので自動的にUIが更新される

## 7. エラーハンドリング

### 7.1 ログ解析エラー
- パースエラーは警告としてログ出力
- アプリケーションは継続動作
- 次の行から処理を再開

### 7.2 データベースエラー
- トランザクションでロールバック
- エラーログ出力
- フロントエンドに通知 (将来実装)

### 7.3 ファイル監視エラー
- ログディレクトリが見つからない場合は警告
- ファイルアクセスエラーはリトライ
- 致命的エラーの場合はアプリ再起動

## 8. パフォーマンス最適化

### 8.1 データベース
- 適切なインデックス
- プリペアドステートメント
- バッチ挿入 (将来)

### 8.2 イベント処理
- 通知不要なイベントは `None` を返す
- 重複イベントの抑制 (upsert使用)
- 軽量なProcessedEvent

### 8.3 フロントエンド
- 必要最小限のデータ再取得
- リストの仮想スクロール (将来)
- 画像の遅延ロード (将来)

## 9. 将来の拡張

### 9.1 リアルタイムスクリーンショット通知
```rust
ProcessedEvent::ScreenshotTaken {
    session_id: i64,
    file_path: String,
}
```
フロントエンドで即座にサムネイル表示

### 9.2 ワールドメタデータ更新通知
```rust
ProcessedEvent::WorldMetadataUpdated {
    session_id: i64,
}
```
ワールド名が後から判明した場合に通知

### 9.3 詳細なプレイヤーイベント
```rust
ProcessedEvent::PlayerAvatarChanged {
    session_id: i64,
    player_id: i64,
}
```
アバター履歴のリアルタイム表示

## 10. ディレクトリ構造

```
VRCJournal/
├── src/                          # フロントエンド (Vue)
│   ├── components/
│   │   ├── common/               # 共通コンポーネント
│   │   │   ├── Button.vue
│   │   │   ├── Card.vue
│   │   │   ├── Modal.vue
│   │   │   ├── Dropdown.vue
│   │   │   └── EmptyState.vue
│   │   ├── views/                # ビューコンポーネント
│   │   │   ├── WorldsView.vue
│   │   │   ├── PeopleView.vue
│   │   │   ├── PhotosView.vue
│   │   │   └── StatsView.vue
│   │   ├── Navigation.vue
│   │   ├── SessionList.vue
│   │   ├── SessionCard.vue
│   │   ├── ScreenshotList.vue
│   │   ├── PlayerList.vue
│   │   ├── ScreenshotModal.vue
│   │   ├── Settings.vue
│   │   ├── ThemeSelector.vue
│   │   └── NotificationContainer.vue
│   ├── composables/              # Vue Composables
│   │   └── useNotifications.ts
│   ├── styles/
│   │   └── theme.css             # デザインシステム
│   ├── types.ts                  # TypeScript型定義
│   ├── App.vue                   # ルートコンポーネント
│   └── main.ts                   # エントリーポイント
│
├── src-tauri/                    # バックエンド (Rust)
│   ├── src/
│   │   ├── db/                   # データベースレイヤー
│   │   │   ├── mod.rs
│   │   │   ├── migrations.rs
│   │   │   └── operations.rs
│   │   ├── parser/               # ログパーサー
│   │   │   ├── mod.rs
│   │   │   ├── log_parser.rs
│   │   │   └── types.rs
│   │   ├── event_processor/      # イベント処理
│   │   │   ├── mod.rs
│   │   │   └── processor.rs
│   │   ├── log_watcher/          # ログ監視
│   │   │   ├── mod.rs
│   │   │   └── watcher.rs
│   │   └── lib.rs                # Tauri Commands
│   └── Cargo.toml
│
└── docs/                         # ドキュメント
    ├── requirements.md           # プロダクト要件定義
    ├── architecture.md           # システムアーキテクチャ (このファイル)
    ├── design-system.md          # デザインシステム (TODO)
    └── development-log.md        # 開発ログ (TODO)
```

## 11. 多言語対応 (i18n)

### 対応言語
- 日本語 (ja) - デフォルト
- 英語 (en)

### 実装方針
- **Vue I18n**: UI文字列の翻訳管理
- **Day.js**: 日付・時刻のローカライズされたフォーマット
- **Tauri OS Plugin**: システムロケールの自動検出
- **LocalStorage**: ユーザーが選択した言語設定の永続化

### 特徴
- システムロケールをデフォルトとし、ユーザーが手動で変更可能
- 言語切り替え時、UI文字列と日付フォーマットが即座に更新
- Vue I18nの`locale` refをリアクティブな依存関係として活用し、computed property内の日付フォーマットも自動更新

### インストーラーの多言語対応

**配布形式**: NSIS インストーラー (`VRCJournal_0.1.0_x64-setup.exe`)

**インストーラーの言語設定**:
- **対応言語**: 英語、日本語
- **言語選択**: インストーラー起動時に言語選択ダイアログが表示される
- **設定の保存**: 選択した言語はレジストリに保存され、次回インストール時にも使用される

**重要な注意点**:
- 一度インストールすると、選択した言語がレジストリに記録される
- 次回インストール時に言語選択ダイアログを再度表示させるには、アンインストール時に「アプリの設定も削除」にチェックを入れる必要がある
- これにより、NSIS設定がレジストリから削除され、次回インストール時に言語選択が表示される

## 12. まとめ

VRCJournalは**イベント駆動アーキテクチャ**と**多言語対応**を中心とした設計です:

1. **ログ変更 → 即座にイベント発火**
2. **データベース保存 → フロントエンド通知**
3. **フロントエンド → リアクティブに更新**
4. **ロケール変更 → UI・日付フォーマット即座に反映**

この設計により:
- ✅ リアルタイムでUIが更新される
- ✅ フロントエンドのポーリング不要
- ✅ バックエンドとフロントエンドの疎結合
- ✅ 拡張性が高い (新しいイベントの追加が容易)
- ✅ 多言語対応 (日英、システムロケール自動検出)
