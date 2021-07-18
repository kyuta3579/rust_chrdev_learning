# Learning MEMO

## chrdev  

# about fp
- UserSlicePtr
  - ユーザランドptrのWrapper？
  - ユーザランドとのデータのやりとりはこの型を介して行うっぽい
  - ReadはUserSlicePtrReader,WriteはUserSlicePtrWriterを用いる
    - 中でCのcopy_to_user()とcopy_from_user()を呼んでいる。
    - read()ではUserSlicePtrWriterで要求されたデータを渡す。write()ではUserSlicePtrReaderで渡されたデータを貰う。
  - read_rawで見ているlen > self.1って不等号逆じゃない？
    - len: 束縛先の配列サイズ、self.1: 入力されたデータサイズ、に見える。
      - もしそうだった場合、「書き込み元のサイズより書き込み先のサイズが小さい場合のみデータがとれる」という処理に見える。
        - つまり書きこむサイズ=>書きこみ先サイズの場合のみデータがとれる。


7/6にnext-linuxに破壊的変更が入ってたので、今のコードは動かない。