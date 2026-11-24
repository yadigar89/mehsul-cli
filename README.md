# Mehsul CLI

Terminaldan idarə olunan sadə məhsul siyahısı proqramı. Rust, clap və serde_json ilə yazılıb.

## Əmrlər

- `elave` : yeni məhsul əlavə edir
- `siyahi` : bütün məhsulları göstərir
- `sil` : məhsulu silir
- `deyis` : məhsulun qiymətini dəyişir

## İşlətmə

```
cargo run -- elave --ad Alma --qiymet 1.5
cargo run -- siyahi
cargo run -- sil --ad Alma
cargo run -- deyis --ad Alma --qiymet 2.0
```