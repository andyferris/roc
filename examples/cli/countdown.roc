app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.9.1/y_Ww7a2_ZGjp0ZTt9Y_pNdSqqMRdMLzHMKfdN8LWidk.tar.br" }

import pf.Stdin
import pf.Stdout
import pf.Task exposing [await, loop]

main =
    _ <- await (Stdout.line "\nLet's count down from 3 together - all you have to do is press <ENTER>.")
    _ <- await Stdin.line
    loop 3 tick

tick = \n ->
    if n == 0 then
        _ <- await (Stdout.line "🎉 SURPRISE! Happy Birthday! 🎂")
        Task.ok (Done {})
    else
        _ <- await (n |> Num.toStr |> \s -> "$(s)..." |> Stdout.line)
        _ <- await Stdin.line
        Task.ok (Step (n - 1))
