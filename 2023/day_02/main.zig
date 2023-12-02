const std = @import("std");

const Inventory = struct {
    red: u32,
    green: u32,
    blue: u32,
};

const Result = struct {
    sum: u32,
    power_sum: u32,
};

pub fn main() !void {
    const budget = Inventory{
        .red = 12,
        .green = 13,
        .blue = 14,
    };

    std.debug.print("input_01: {}\n", .{try solve("input_01.txt", budget)});
    std.debug.print("input_02: {}\n", .{try solve("input_02.txt", budget)});
}

fn solve(path: []const u8, budget: Inventory) !Result {
    const file = try std.fs.cwd().openFile(
        path,
        .{},
    );
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();
    var buf: [256]u8 = undefined;

    var i: u32 = 0;
    var sum: u32 = 0;
    var power_sum: u32 = 0;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        i += 1;
        if (processLine(line, budget)) {
            sum += i;
        }
        const min_inventory = calculateMinInventory(line);
        const power = min_inventory.red * min_inventory.green * min_inventory.blue;
        power_sum += power;
    }

    return Result{
        .sum = sum,
        .power_sum = power_sum,
    };
}

// line of the form: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
// returns true if the game is possible for the given budget
fn processLine(line: []const u8, inventory: Inventory) bool {
    var iter = std.mem.splitAny(u8, line, ":, ");
    _ = iter.next(); // skip "Game"
    _ = iter.next(); // skip game number
    _ = iter.next(); // skip empty string

    var budget = inventory;

    while (iter.next()) |word| {
        if (word.len == 0) {
            continue;
        }

        const count = std.fmt.parseUnsigned(u32, word, 10) catch 0;

        const color = iter.next() orelse break;

        if (std.mem.startsWith(u8, color, "red")) {
            if (budget.red < count) {
                return false;
            }
            budget.red -= count;
        } else if (std.mem.startsWith(u8, color, "green")) {
            if (budget.green < count) {
                return false;
            }
            budget.green -= count;
        } else if (std.mem.startsWith(u8, color, "blue")) {
            if (budget.blue < count) {
                return false;
            }
            budget.blue -= count;
        } else {
            return false;
        }

        if (std.mem.endsWith(u8, color, ";")) {
            // Reset the budget for the next turn
            budget = inventory;
        }
    }

    return true;
}

// line of the form: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
// calculates the minimum inventory needed to play the game
fn calculateMinInventory(line: []const u8) Inventory {
    // This includes `;` since turns don't mean anything for this calculation
    var iter = std.mem.splitAny(u8, line, ":,; ");
    _ = iter.next(); // skip "Game"
    _ = iter.next(); // skip game number
    _ = iter.next(); // skip empty string

    var budget = Inventory{
        .red = 0,
        .green = 0,
        .blue = 0,
    };

    while (iter.next()) |word| {
        if (word.len == 0) {
            continue;
        }

        const count = std.fmt.parseUnsigned(u32, word, 10) catch 0;

        const color = iter.next() orelse break;

        if (std.mem.eql(u8, color, "red")) {
            budget.red = @max(budget.red, count);
        } else if (std.mem.eql(u8, color, "green")) {
            budget.green = @max(budget.green, count);
        } else if (std.mem.eql(u8, color, "blue")) {
            budget.blue = @max(budget.blue, count);
        } else {
            return Inventory{
                .red = 0,
                .green = 0,
                .blue = 0,
            };
        }
    }

    return budget;
}
