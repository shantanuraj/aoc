function solve(input: string) {
  const lines = input.split("\n").slice(0, -1);
  console.log(lines);
}

console.log("input_01.txt", solve(await Bun.file("input_01.txt").text()));
