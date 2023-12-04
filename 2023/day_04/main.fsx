let getTokens (line: string) =
    line.Split([|' '|]) |> Array.filter (fun s -> s.Length > 0) |> Array.map (fun s -> s.Trim())

// Get numbers from line of the form
// `Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53`
let getSum (line: string) =
    let line = line.[line.IndexOf(": ") + 2..]

    let left = Set.ofArray (line.[..line.IndexOf("|") - 1] |> getTokens)
    let right = Set.ofArray (line.[line.IndexOf("|") + 2..] |> getTokens)

    // Count how many numbers on the right are also on the left
    let count = Set.count (Set.intersect left right)

    pown 2 (count - 1)

let solve (input: string) =
    input.Split([|'\n'|]) |> Array.map getSum |> Array.sum

// Returns function that takes a line and updates the array
let cardCount (copies: int[]) =
    fun (i: int) ->
        fun(line: string) ->
            let line = line.[line.IndexOf(": ") + 2..]

            let left = Set.ofArray (line.[..line.IndexOf("|") - 1] |> getTokens)
            let right = Set.ofArray (line.[line.IndexOf("|") + 2..] |> getTokens)

            // Count how many numbers on the right are also on the left
            let count = Set.count (Set.intersect left right)

            // For the next `count` elements, increment the number of copies by current line's number of copies
            for j in i + 1 .. i + count do
                copies.[j] <- copies.[j] + copies.[i]

            copies.[i]

let solve2 (input: string) =
    let lines = input.Split([|'\n'|])

    // Initialize an array with the same length as the input, set all values to 1
    let mutable copies = Array.create lines.Length 1

    lines |> Array.mapi (cardCount copies) |> Array.sum

let input_01 = System.IO.File.ReadAllText("input_01.txt")
let input_02 = System.IO.File.ReadAllText("input_02.txt")

printfn "input_01.txt -- %d" (solve input_01)
printfn "input_02.txt -- %d" (solve input_02)

printfn "input_01.txt -- %d" (solve2 input_01)
printfn "input_02.txt -- %d" (solve2 input_02)
