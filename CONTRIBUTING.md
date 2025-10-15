# 開発者向け情報

VRCJournalの開発に興味を持っていただき、ありがとうございます。

## 開発環境のセットアップ

### 必要なもの

- [Node.js](https://nodejs.org/) (v18以上)
- [Rust](https://rustup.rs/)
- Windows 10/11

### セットアップ

```bash
# リポジトリのクローン
git clone https://github.com/sakamotch/VRCJournal.git
cd VRCJournal

# 依存関係のインストール
npm install

# 開発サーバーの起動
npm run tauri dev
```

初回起動時はRustの依存関係のビルドに時間がかかります。

### ビルド

```bash
npm run tauri build
```

ビルドされたファイルは `src-tauri/target/release/` に出力されます。

## プロジェクト構成

```
VRCJournal/
├── src/                    # Vue 3 フロントエンド
│   └── App.vue            # メインコンポーネント
│
├── src-tauri/             # Rust バックエンド
│   ├── src/
│   │   ├── parser/        # VRChatログパーサー
│   │   ├── log_watcher/   # ログファイル監視
│   │   ├── event_processor/ # イベント処理
│   │   ├── db/            # データベース操作
│   │   └── lib.rs         # Tauriコマンド
│   │
│   └── migrations/        # SQLiteスキーマ
│       └── 001_initial_schema.sql
│
└── docs/
    └── architecture.md    # アーキテクチャ設計
```

## 技術スタック

- **フロントエンド**: Vue 3 + TypeScript + Vite
- **バックエンド**: Rust + Tauri 2.x
- **データベース**: SQLite (rusqlite)

## バグ報告・機能提案

Issueでお気軽にお知らせください。

- **バグ報告**: 再現手順と期待される動作を記載
- **機能提案**: どんな機能か、なぜ必要かを記載

## 参考資料

- [アーキテクチャ設計](docs/architecture.md)
- [Tauri ドキュメント](https://v2.tauri.app/)
- [Vue 3 ドキュメント](https://ja.vuejs.org/)

---

ご協力ありがとうございます！
