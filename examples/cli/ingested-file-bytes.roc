app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.9.1/y_Ww7a2_ZGjp0ZTt9Y_pNdSqqMRdMLzHMKfdN8LWidk.tar.br" }

import pf.Stdout
import "../../LICENSE" as license : _ # A type hole can also be used here.

main =
    # Due to how license is used, it will be a List U8.
    license
    |> List.map Num.toU64
    |> List.sum
    |> Num.toStr
    |> Stdout.line
