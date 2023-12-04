let getTokens (line: string) =
    line.Split([|' '|]) |> Array.filter (fun s -> s.Length > 0) |> Array.map (fun s -> s.Trim())

// Get numbers from line of the form
// `Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53`
let getSum (line: string) =
    let line = line.[line.IndexOf(": ") + 2..]

    let left = line.[..line.IndexOf("|") - 1] |> getTokens
    let right = line.[line.IndexOf("|") + 2..] |> getTokens

    // Count how many numbers on the right are also on the left
    let count = right |> Array.filter (fun n -> left |> Array.exists (fun m -> m = n)) |> Array.length
    pown 2 (count - 1)

let solve (input: string) =
    input.Split([|'\n'|]) |> Array.map getSum |> Array.sum

let input_01 = System.IO.File.ReadAllText("input_01.txt")
printfn "input_01.txt -- %d" (solve input_01)
let input_02 = System.IO.File.ReadAllText("input_02.txt")
printfn "input_02.txt -- %d" (solve input_02)