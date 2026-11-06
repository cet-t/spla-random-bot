#[derive(Debug, Clone, Copy, PartialEq, Eq, poise::ChoiceParameter)]
pub enum MainFilter {
    #[name = "シューター"]
    Shooter,
    #[name = "リールガン"]
    Reelgun,
    #[name = "ローラー"]
    Roller,
    #[name = "チャージャー"]
    Charger,
    #[name = "スロッシャー"]
    Slosher,
    #[name = "スピナー"]
    Spinner,
    #[name = "ブラスター"]
    Blaster,
    #[name = "フデ"]
    Brush,
    #[name = "マニューバー"]
    Maneuver,
    #[name = "シェルター"]
    Shelter,
    #[name = "ストリンガー"]
    Stringer,
    #[name = "ワイパー"]
    Wiper,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, poise::ChoiceParameter)]
pub enum SubFilter {
    #[name = "スプラッシュボム"]
    Splashbomb,
    #[name = "キューバンボム"]
    KyubanBomb,
    #[name = "カーリングボム"]
    CurlingBomb,
    #[name = "ポイズンミスト"]
    PoisonMist,
    #[name = "スプラッシュシールド"]
    SplashShield,
    #[name = "ラインマーカー"]
    LineMarker,
    #[name = "クイックボム"]
    QuickBomb,
    #[name = "ロボットボム"]
    RobotBomb,
    #[name = "スプリンクラー"]
    Sprinkler,
    #[name = "タンサンボム"]
    TansanBomb,
    #[name = "トーピード"]
    Torpedo,
    #[name = "トラップ"]
    Trap,
    #[name = "ジャンプビーコン"]
    JumpBeacon,
    #[name = "ポイントセンサー"]
    PointSensor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, poise::ChoiceParameter)]
pub enum SpecialFilter {
    #[name = "カニタンク"]
    CrabTank,
    #[name = "グレートバリア"]
    BigBubbler,
    #[name = "ショクワンダー"]
    Zipcaster,
    #[name = "ウルトラショット"]
    Trizooka,
    #[name = "メガホンレーザー5.1ch"]
    KillerWail,
    #[name = "キューインキ"]
    InkVac,
    #[name = "トリプルトルネード"]
    TripleInkstrike,
    #[name = "エナジースタンド"]
    Tacticooler,
    #[name = "ホップソナー"]
    WaveBreaker,
    #[name = "サメライド"]
    Reefslider,
    #[name = "マルチミサイル"]
    TentaMissiles,
    #[name = "ジェットパック"]
    Inkjet,
    #[name = "アメフラシ"]
    InkStorm,
    #[name = "ウルトラハンコ"]
    UltraStamp,
    #[name = "ナイスダマ"]
    BooyahBomb,
    #[name = "テイオウイカ"]
    KrakenRoyal,
    #[name = "デコイチラシ"]
    SuperChump,
    #[name = "スミナガシート"]
    SplattercolorScreen,
    #[name = "ウルトラチャクチ"]
    TripleSplashdown,
}
