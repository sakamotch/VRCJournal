# VRCJournal - デザインシステム

## 1. デザインコンセプト

**「思い出を彩るモダンなジャーナル」**

- **モダン**: グラデーション、ブラー、洗練されたアニメーション
- **シック**: 落ち着いたIndigo/Violetカラーパレット
- **直感的**: アイコンと視覚的フィードバック
- **心地よい**: スムーズなトランジション

## 2. デザイントークンシステム

### 2.1 3層アーキテクチャ

**定義場所**: [`src/styles/theme.css`](../src/styles/theme.css)

```
Primitive (色のパレット)
  ↓ 参照
Alias (用途別トークン)
  ↓ 参照
Semantic (コンポーネント固有)
```

**利点:**
- 一箇所の変更で全体に反映
- 一貫性のある配色
- 保守性の向上

### 2.2 主要カラー
- **メインカラー**: Indigo
- **アクセント**: Violet
- **ベースカラー**: Warm Gray
- **インタラクティブ**: Blue
- **フィードバック**: Green (成功), Red (エラー), Orange (警告), Blue (情報)

### 2.3 CSS変数とcolor-mix()
- CSS Custom Properties によるトークン管理
- `color-mix()` による動的なカラーミキシング
- グラデーションの多用（背景、ホバー、ボーダー）

## 3. ダークモード対応

### 3.1 自動切り替え
- システム設定 (`prefers-color-scheme`) に追従
- 明示的なライト/ダーク切り替えも可能 (`data-theme` 属性)

### 3.2 実装方法
- Aliasトークンを使用することで自動的にダークモード対応
- Primitiveカラーの直接使用を避ける

## 4. コンポーネントによる共通化

### 4.1 共通コンポーネント
**定義場所**: `src/components/common/`

- **Button**: アクション用ボタン（バリアント: primary, secondary, ghost）
- **Card**: コンテナ（オプション: hoverable, clickable）
- **Modal**: オーバーレイダイアログ（Teleport使用）
- **Dropdown**: ドロップダウンメニュー
- **EmptyState**: プレースホルダー状態

### 4.2 設計原則
- 再利用性を重視
- Props による柔軟な制御
- 一貫したスタイリング（グラデーション、ホバーエフェクト）

### 4.3 共通パターン
- **グラデーション背景**: モダンな印象
- **::before疑似要素**: ホバーエフェクト実装
- **z-index管理**: 疑似要素とコンテンツの重なり制御
- **滑らかなトランジション**: 全インタラクションにアニメーション

## 5. アイコンシステム

### 5.1 ライブラリ
**Lucide Vue** - https://lucide.dev/

### 5.2 使用理由
- 一貫したデザイン言語
- Vue 3対応
- Tree-shaking対応
- カスタマイズ可能（サイズ、stroke-width）

### 5.3 主要アイコン
- ナビゲーション: Clock, Globe, Users, Camera, BarChart3
- アクション: ExternalLink, Folder, X, Settings, ChevronDown
- 通知: CheckCircle2 (成功), XCircle (エラー), Info (情報), AlertTriangle (警告)

## 6. まとめ

VRCJournalのデザインシステムは以下の特徴を持ちます:

1. **3層デザイントークン** - 一貫性と保守性
2. **Indigo/Violetグラデーション** - モダンで洗練された印象
3. **自動ダークモード** - システム設定に追従
4. **共通コンポーネント** - 再利用性とDRY原則
5. **Lucide Vueアイコン** - 一貫したビジュアル言語

**詳細**: デザイントークンの全定義は [`src/styles/theme.css`](../src/styles/theme.css) を参照
