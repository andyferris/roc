app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.9.1/y_Ww7a2_ZGjp0ZTt9Y_pNdSqqMRdMLzHMKfdN8LWidk.tar.br" }

import pf.Stdout
import "ingested-file.roc" as ownCode : Str

main =
    Stdout.line "\nThis roc file can print its own source code. The source is:\n\n$(ownCode)"
