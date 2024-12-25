# yewの環境構築方法
cargo install cargo-binstall
cargo binstall trunk

# yewの開発用ビルド方法(生成されたdistディレクトリ内のファイルはデプロイしても大丈夫)
trunk serve --open

# yewの本番環境用ビルド方法
trunk build --release