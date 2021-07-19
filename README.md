# Learning MEMO
next-20210719対応
## chrdev  

# about fp
- `UserSlicePtr`構造体
  - ユーザランドptrのWrapper？
  - ユーザランドとのデータのやりとりはこの型を介して行うっぽい
  - Readは`UserSlicePtrReader`,Writeは`UserSlicePtrWriter`を用いる
    - 中でCのcopy_to_user()とcopy_from_user()を呼んでいる。
    - fpのread()では`UserSlicePtrWriter`で要求されたデータを書く。fpのwrite()では`UserSlicePtrReader`で渡されたデータを読む。
    - next-20210706から`IoBufferWriter`,`IoBufferReader`トレイトを実装する形になった。基本的な動作は変わらない。
  - `read_raw`で見ているlen > self.1って不等号逆じゃない？
    - len: 束縛先の配列サイズ、self.1: 入力されたデータサイズ、に見える。
      - もしそうだった場合、「書き込み元のサイズより書き込み先のサイズが小さい場合のみデータがとれる」という処理に見える。
        - つまり書きこむサイズ=>書きこみ先サイズの場合のみデータがとれる。
- `IoBufferWriter`、`IoBufferReader`トレイト
  - IOとやりとりするデータはこのトレイトを介して行う形式になったっぽい
  - 今まで`UserSlicePtrWriter`と`UserSlicePtrReader`内で実装されていた関数を移管している。
  - 他構造体でもユーザ空間とのデータやりとり出来るようにしている？
- `Cstr`構造体
  - CのstrをRustで用いるための構造体
  - `c_str`マクロで文字列を変換して、`as_bytes`メソッドでアドレスを取り出すのが基本の使い方っぽい
  - あらかじめCstr構造体の領域を確保しておくことは可能だが、空の動的確保は出来ないっぽい？
    - 文字列の長さが決定してから`c_str`マクロを使用するべきっぽい
  - byteで表されている文字列(&[u8])からは`from_bytes_with_nul_unwrap`で&CStrを得られる