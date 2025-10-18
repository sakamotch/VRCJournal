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
- **ファイル監視**: 独自ポーリング (VRChat側がバッファリングしているためnotifyでは対応不可)

### プラットフォーム

- **対応OS**: Windows

## 2. アーキテクチャ原則

### 2.1 イベント駆動アーキテクチャ

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
4. ファイル監視を開始 (独自ポーリング)

// 実行時
1. ファイルサイズ変更を検知 (500msポーリング)
2. 新しい行のみを読み取り
3. LogParserに渡す
4. ファイル位置をデータベースに保存
```

**重要な実装:**
- ファイル位置 (byte offset) をデータベースに永続化
- アプリ再起動後も続きから読み取り
- 複数のログファイルを同時監視

**VRChatのバッファリング対策:**

VRChatはログファイルをメモリバッファに書き込み、即座にフラッシュしない場合があります。このため、OSのファイルシステムイベント（`notify`クレートの`RecommendedWatcher`）では変更を検知できないことがあります。

**解決策: 独自ポーリング実装**
```rust
// 500msごとにファイルサイズを直接チェック
loop {
    let current_size = fs::metadata(&file_path)?.len();
    if current_size > previous_size {
        // ファイルが増加 → 新しいデータを読み込み
        handle_file_change(&file_path);
    }
    thread::sleep(Duration::from_millis(500));
}
```

この方式により、VRChatのバッファリングに関係なく、ファイルサイズの変更を確実に検知できます。

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
    current_instance_id: Option<i64>,          // 現在のインスタンス
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
    InstanceCreated { instance_id: i64 },        // 新インスタンス作成
    InstanceEnded { instance_id: i64, ended_at: String }, // インスタンス終了
    PlayerJoined { instance_id: i64 },          // プレイヤー参加
    PlayerLeft { instance_id: i64 },            // プレイヤー退出
}
```

**イベント処理ルール:**

| LogEvent | データベース操作 | ProcessedEvent | 備考 |
|----------|------------------|----------------|------|
| UserAuthenticated | ローカルプレイヤー作成/更新 | LocalPlayerUpdated | アカウント切り替え検知 |
| JoiningWorld | インスタンス作成 | InstanceCreated | 前インスタンスは自動的にinterrupted |
| EnteringRoom | インスタンスのworld_name更新 | なし | メタデータ補完のみ |
| PlayerJoined | プレイヤー作成/更新 + instance_players追加 | PlayerJoined | プレイヤー数更新 |
| DestroyingPlayer (自分) | インスタンス終了 + 全員退出処理 | InstanceEnded | インスタンス完了 |
| DestroyingPlayer (他) | instance_playersのleft_at更新 | PlayerLeft | プレイヤー数更新 |
| AvatarChanged | アバター記録 + avatar_usages更新 | なし | 将来的にアバター履歴機能で使用 |
| ScreenshotTaken | スクリーンショット記録 | なし | インスタンス詳細で表示 |

### 3.4 Database (バックエンド)
**責務**: データの永続化とクエリ

**スキーマ設計:**
- `players` - プレイヤー情報 (is_local=1がローカルプレイヤー)
- `instances` - インスタンス情報
- `instance_players` - インスタンス参加プレイヤー (joined_at, left_at)
- `avatars` - アバター情報
- `avatar_usages` - インスタンス中のアバター使用履歴
- `screenshots` - スクリーンショット
- `player_name_history` - プレイヤー名前変更履歴
- `log_files` - 処理済みログファイル位置

**重要なインデックス:**
- `instances.player_id` - アカウント別インスタンス取得
- `instance_players.instance_id` - インスタンス内プレイヤー取得
- `screenshots.instance_id` - インスタンス内スクリーンショット取得

### 3.5 Frontend (Vue)
**責務**: UIレンダリングとユーザーインタラクション

**イベントリスナー設定:**
```typescript
// App.vue (起動時に設定)
onMounted(async () => {
  // 初回データ読み込み
  await loadUsers();
  await loadInstances();

  // イベントリスナー登録
  unlistenFn = await listen<ProcessedEvent>("log-event", (event) => {
    const processedEvent = event.payload;

    switch (processedEvent.type) {
      case "LocalPlayerUpdated":
        loadUsers();  // アカウントリスト再取得
        break;

      case "InstanceCreated":
        loadInstances();  // インスタンスリスト再取得
        break;

      case "InstanceEnded":
        // インスタンスのステータスをローカルで更新
        updateInstanceStatus(processedEvent.instance_id, 'completed');
        break;

      case "PlayerJoined":
      case "PlayerLeft":
        // 該当インスタンスのプレイヤー数を再取得
        refreshInstancePlayerCount(processedEvent.instance_id);
        break;
    }
  });
});
```

**データ取得ルール:**
- **初回ロード時**: `get_local_users()`, `get_instances()` を呼び出し
- **イベント受信時**: 必要に応じて再取得
- **ページ切り替え時**: `get_instances()` を必要なら再取得
- **それ以外**: バックエンドからのプッシュを待つ

## 4. データフロー詳細

### 4.1 アプリ起動シーケンス

**重要**: バックエンドとフロントエンドは非同期に起動するため、起動同期の仕組みが必要です。

#### バックエンド起動フロー（別スレッドで実行）
```rust
1. Tauriアプリ起動
   ↓
2. データベース初期化 (migrate)
   ↓
3. AppState初期化
   - db: Database インスタンス
   - event_processor: EventProcessor インスタンス
   - backend_ready: false (準備未完了)
   ↓
4. バックグラウンドスレッド起動
   ├─ EventProcessor初期化
   │  - データベースから最新のローカルプレイヤー取得
   │  - 進行中のセッション復元
   │  - セッション参加プレイヤーのマッピング復元
   ↓
5. LogWatcher起動
   ├─ 処理済みログファイル位置を取得 (DBから)
   ├─ 全ログファイルを初回読み込み
   │  - 差分のみをパース（file_positionから続き）
   │  - LogEvent → EventProcessor → DB保存
   │  - この時点ではフロントエンドへemitしない
   ├─ ファイル位置をDBに保存
   ↓
6. ファイル監視開始（独自ポーリング）
   - 500msごとにファイルサイズをチェック
   - VRChatのバッファリング対策
   ↓
7. backend_ready = true 設定
   ↓
8. "backend-ready" イベント送信
   ↓
9. 監視ループ開始
   - 新しいログイベントをemit
```

#### フロントエンド起動フロー（Vue onMounted）
```typescript
1. App.vueマウント
   ↓
2. バックエンド準備状態を確認
   - is_backend_ready() APIを呼び出し
   ├─ true の場合（F5リロードなど）
   │  └→ 即座に初期データ取得へ
   ↓
3. "backend-ready" イベントリスナー登録
   - バックエンドからの準備完了通知を待機
   - 重複実行を防ぐため isBackendReady フラグでチェック
   ↓
4. "log-event" イベントリスナー登録
   - ログイベントの通知を受け取る準備
   ↓
5. "backend-ready" イベント受信 または 既に準備完了
   ↓
6. 初回データ取得
   - loadUsers() - get_local_users()
   - loadInstances() - get_instances()
   ↓
7. UI表示・待機状態
   - インスタンス一覧表示
   - リアルタイム更新待機
```

#### 起動同期の仕組み

**問題**: バックエンドとフロントエンドは非同期に起動するため、フロントエンドが先に起動してデータを取得しようとすると、バックエンドの初期化が完了していない可能性がある。

**解決策**: `backend_ready` フラグと二段階チェック

1. **AppState に `backend_ready` フラグを追加**
   ```rust
   pub struct AppState {
       db: Arc<Mutex<db::Database>>,
       event_processor: Arc<Mutex<EventProcessor>>,
       backend_ready: Arc<Mutex<bool>>,  // 準備完了フラグ
   }
   ```

2. **`is_backend_ready` コマンド**
   ```rust
   #[tauri::command]
   async fn is_backend_ready(state: tauri::State<'_, AppState>) -> Result<bool, String> {
       Ok(*state.backend_ready.lock().unwrap())
   }
   ```

3. **フロントエンドの二段階チェック**
   ```typescript
   onMounted(async () => {
     // 1. 既に準備完了かチェック（F5リロード対策）
     const ready = await invoke<boolean>("is_backend_ready");
     if (ready) {
       isBackendReady.value = true;
       await loadUsers();
       await loadInstances();
     }

     // 2. 準備完了イベントを待機（初回起動用）
     unlistenReadyFn = await listen("backend-ready", async () => {
       if (!isBackendReady.value) {
         isBackendReady.value = true;
         await loadUsers();
         await loadInstances();
       }
     });
   });
   ```

#### 起動シナリオ別の動作

| シナリオ | バックエンド状態 | フロントエンド動作 |
|---------|----------------|------------------|
| **初回起動** | ログパース中 | `is_backend_ready() = false` → イベント待機 → `backend-ready` 受信 → データ取得 |
| **F5リロード** | 既に起動済み | `is_backend_ready() = true` → 即座にデータ取得 |
| **DBあり起動** | 差分パース中 | `is_backend_ready() = false` → イベント待機 → `backend-ready` 受信 → データ取得 |

**利点**:
- ✅ フロントエンドは確実にバックエンド準備完了後にデータ取得
- ✅ F5リロードでもローディング画面で止まらない
- ✅ 初回起動時も適切に待機
- ✅ バックエンドの初期化時間が可視化される（isBackendReady フラグ）

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
   - InstanceCreated → インスタンスリスト再取得
   - InstanceEnded → インスタンスステータス更新
   - PlayerJoined/Left → プレイヤー数更新
```

### 4.3 インスタンス作成フロー
```
VRChatログ: "[Behaviour] Joining wrld_xxx:12345~region(us)~..."
   ↓
LogParser: LogEvent::JoiningWorld
   ↓
EventProcessor:
   1. 前のインスタンスを interrupted にする (もしあれば)
   2. 新しいインスタンスをデータベースに作成
   3. current_instance_id を更新
   4. player_ids, pending_avatars をクリア
   5. ProcessedEvent::InstanceCreated を返す
   ↓
Tauri emit: "log-event" { type: "InstanceCreated", instance_id: 123 }
   ↓
フロントエンド:
   1. loadInstances() を呼び出し
   2. インスタンスリストを再レンダリング
   3. 新しいインスタンスがタイムラインのトップに表示される
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
   3. instance_players に参加記録を追加
   4. player_ids マッピングに追加
   5. pending_avatars があれば記録
   6. ProcessedEvent::PlayerJoined を返す
   ↓
Tauri emit: "log-event" { type: "PlayerJoined", instance_id: 123 }
   ↓
フロントエンド:
   1. 該当インスタンスのプレイヤー数を再取得 (軽量)
   2. InstanceCard のプレイヤー数バッジを更新
```

## 5. 非イベント駆動部分 (例外)

以下の機能はユーザーアクション時に**Command呼び出し**を行います:

### 5.1 初回データロード
- `get_local_users()` - アプリ起動時
- `get_instances()` - アプリ起動時、アカウント切り替え時

### 5.2 詳細情報の取得
- `get_instance_by_id(instance_id)` - インスタンス詳細ビュー表示時
- `get_instance_players(instance_id)` - インスタンス詳細ビュー表示時
- `get_instance_screenshots(instance_id)` - インスタンス詳細ビュー表示時

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
    current_instance_id: Option<i64>,
    player_ids: HashMap<String, i64>,
    pending_avatars: HashMap<String, (i64, DateTime<Utc>)>,
}
```

**状態遷移:**
- `UserAuthenticated` → current_local_player_id 更新
- `JoiningWorld` → current_instance_id 更新、マップクリア
- `PlayerJoined` → player_ids に追加
- `DestroyingPlayer (自分)` → current_instance_id クリア、マップクリア

### 6.2 フロントエンド状態 (Vue Reactivity)
```typescript
// App.vue
const localUsers = ref<LocalUser[]>([]);     // アカウントリスト
const instances = ref<Instance[]>([]);         // インスタンスリスト
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
    instance_id: i64,
    file_path: String,
}
```
フロントエンドで即座にサムネイル表示

### 9.2 ワールドメタデータ更新通知
```rust
ProcessedEvent::WorldMetadataUpdated {
    instance_id: i64,
}
```
ワールド名が後から判明した場合に通知

### 9.3 詳細なプレイヤーイベント
```rust
ProcessedEvent::PlayerAvatarChanged {
    instance_id: i64,
    player_id: i64,
}
```
アバター履歴のリアルタイム表示

## 10. ディレクトリ構造

```
VRCJournal/
├── src/                          # フロントエンド (Vue)
│   ├── components/
│   │   ├── common/               # 共通UIコンポーネント
│   │   │   ├── Button.vue
│   │   │   ├── Card.vue
│   │   │   ├── Modal.vue
│   │   │   ├── Dropdown.vue
│   │   │   └── EmptyState.vue
│   │   ├── views/                # ページビューコンポーネント
│   │   │   ├── InstancesView.vue
│   │   │   ├── WorldsView.vue
│   │   │   ├── PeopleView.vue
│   │   │   ├── PhotosView.vue
│   │   │   └── StatsView.vue
│   │   ├── features/             # 機能別コンポーネント
│   │   │   ├── instance/
│   │   │   │   └── InstanceCard.vue
│   │   │   ├── player/
│   │   │   │   └── PlayerList.vue
│   │   │   └── screenshot/
│   │   │       ├── ScreenshotList.vue
│   │   │       └── ScreenshotModal.vue
│   │   ├── layout/               # レイアウトコンポーネント
│   │   │   ├── Navigation.vue
│   │   │   ├── Sidebar.vue
│   │   │   └── NotificationContainer.vue
│   │   └── settings/             # 設定関連コンポーネント
│   │       ├── Settings.vue
│   │       ├── ThemeSelector.vue
│   │       └── LanguageSelector.vue
│   ├── composables/              # Vue Composables
│   │   └── useNotifications.ts
│   ├── stores/                   # 状態管理
│   │   └── themeStore.ts
│   ├── types/                    # TypeScript型定義
│   │   └── index.ts
│   ├── utils/                    # ユーティリティ関数
│   │   ├── dayjs-config.ts
│   │   └── formatters.ts
│   ├── i18n/                     # 多言語対応
│   │   ├── index.ts
│   │   └── locales/
│   │       ├── ja.ts
│   │       └── en.ts
│   ├── styles/                   # スタイル定義
│   │   ├── primitives.css
│   │   ├── theme.css
│   │   └── themes/
│   │       ├── light.css
│   │       ├── dark.css
│   │       ├── cyberpunk.css
│   │       ├── aurora.css
│   │       └── pastel.css
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
