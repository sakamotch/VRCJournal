# VRCJournal

自分自身の活動を記録するためのログ管理ツールです。
他人の行動を追跡する機能は一切持たず、「自分がどこで、誰と、何をしたか」を振り返るための個人用ツールとして開発されています。


## 特徴(予定)

- ログの収集範囲は自分が参加したインスタンスのみ。
- 他者が別のインスタンスにいる情報、活動履歴などは一切記録しない。
- タグ付け・メモ機能（自分用）。
- 日付ごとのフィルタリング・検索。
- ローカル実行（外部通信なし、ログイン不要）。


## 注意事項

- このツールは個人のログ管理を目的としています。
- 他ユーザーの行動追跡を目的とした利用は禁止します。
- ログファイルはローカルで処理され、外部への共有は一切行いません。


## 開発環境のセットアップ（Docker）

### 必要なもの

- Docker
- Docker Compose

### 環境構築手順

1. **リポジトリをクローン**
   ```bash
   git clone <repository-url>
   cd VRCJournal
   ```

2. **Dockerコンテナをビルド・起動**
   ```bash
   docker-compose up -d --build
   ```

3. **コンテナに入る**
   ```bash
   docker-compose exec tauri-dev bash
   ```

4. **Tauriプロジェクトを初期化（初回のみ）**
   ```bash
   # コンテナ内で実行
   npm create tauri-app
   # または既存のフロントエンドフレームワークを使用
   ```

5. **開発サーバーを起動**
   ```bash
   # コンテナ内で実行
   npm run tauri dev
   ```

### よく使うコマンド

```bash
# コンテナを起動
docker-compose up -d

# コンテナを停止
docker-compose down

# コンテナに入る
docker-compose exec tauri-dev bash

# ログを確認
docker-compose logs -f

# コンテナを再ビルド
docker-compose up -d --build
```

### トラブルシューティング

- **GUIアプリが表示されない場合**: Docker環境ではGUIの表示に制限があります。開発時はWebビュー機能を使用するか、ホスト環境でのビルドを検討してください。
- **ポートが使用中の場合**: [docker-compose.yml](docker-compose.yml) のポート設定を変更してください。
