## Rust の & による参照を考察するためのもの
現時点では、main() には意味がほとんどないため「cargo run」や「cargo build」をする意義はない。  
**「cargo test -- --nocapture」** を実行した場合に表示される結果を理解することが目的となっている。  

cargo test -- --nocapture によって実行されるのは、「main.rs」の 151 ～ 196行目 の内容。  

「Cargo.toml」の、[features] の default の内容を書きかえることにより、他の test を実行できるようになっている。  
[features] を一々書きかえるのが面倒な場合は、default の行を「#」でコメントアウトして、  
**「cargo test -F test_1 -- --nocapture」** とするとテスト内容を簡単に変更できる。

## プログラム中で使われている Pin について
フィールドに、アドレスを１つだけ持つ単純な構造体。  
役割としては、**「Pin で修飾したものは、メモリ内を移動させることを禁止する」（コピーも禁止）** ということをコンパイラに伝えるだけのもの。  
（多分、合ってると思うけど(^^;。違ってたら教えて！）
