import sys.io.File;
import haxe.io.Eof;

class Main {
	static final LO:Int = 0;
	static final HI:Int = 1;
	static final file_paths:Map<String, String> = [
		"0" => "data/cohen00.txt",
		"1" => "data/cohen01.txt",
		"2" => "data/cohen02.txt",
		"3" => "data/cohen03.txt",
		"4" => "data/cohen04.txt",
		"5" => "data/cohen05.txt",
		"6" => "data/cohen06.txt",
		"7" => "data/cohen07.txt",
		"8" => "data/cohen08.txt",
		"9" => "data/cohen09.txt",
		"10" => "data/cohen10.txt",
		"11" => "data/cohen11.txt",
		"12" => "data/cohen12.txt",
	];

	static function main():Void {
		while (menu() != "13") {};
	}

	static function menu():String {
		show_options();
		var option:String = read_user_input();
		switch option {
			case "1":
				process_one_file();
			case "2":
				process_all_files();
				option = "13";
			case "3":
				Sys.println("");
				Sys.println(">>> Bye ^_~");
				option = "13";
			case _:
				Sys.println("");
				Sys.println("Invalid option. Try again.");
				Sys.println("");
		}
		return option;
	}

	static function show_options():Void {
		Sys.println("Hi! What do you want to do?");
		Sys.println("1 - Run one file");
		Sys.println("2 - Run all files");
		Sys.println("3 - Quit");
	}

	static function read_user_input():String {
		Sys.print("Input selected file number: ");
		return Sys.stdin().readLine();
	}

	static function process_one_file():Void {
		Sys.println("Available files:");
		show_files_options();
		final index:String = read_user_input();
		final ip_list:Array<Array<Int>> = read_file(file_paths[index]);
		final file_name = file_paths[index].split("/").pop();
		ip_list.sort(function(x:Array<Int>, y:Array<Int>):Int {
			return if (x[LO] < y[LO]) -1 else 1;
		});
		final length = least_possible_size(ip_list);

		Sys.println("");
		Sys.println(">>> " + file_name + " => " + length);
		Sys.println("");
	}

	static function show_files_options():Void {
		Sys.println("0 - cohen00.txt");
		Sys.println("1 - cohen01.txt");
		Sys.println("2 - cohen02.txt");
		Sys.println("3 - cohen03.txt");
		Sys.println("4 - cohen04.txt");
		Sys.println("5 - cohen05.txt");
		Sys.println("6 - cohen06.txt");
		Sys.println("7 - cohen07.txt");
		Sys.println("8 - cohen08.txt");
		Sys.println("9 - cohen09.txt");
		Sys.println("10 - cohen10.txt");
		Sys.println("11 - cohen11.txt");
		Sys.println("12 - cohen12.txt");
	}

	static function read_file(file_path: String): Array<Array<Int>> {
		var aux: Array<Array<Int>> = new Array<Array<Int>>();

		try {
			var tmp = File.read(file_path);
			while (true) {
				aux.push(
					tmp.readLine()
					.split("-")
					.map(
						function(string_ip) {
							return Std.parseInt(string_ip);
						}
					)
				);
			}
		} catch (excException:Eof) {}

		return aux;
	}

	static function least_possible_size(ip_list:Array<Array<Int>>):Int {
		var blocked_ranges:Array<Array<Int>> = new Array<Array<Int>>();

		for (ip_pair in ip_list) {
			final LAST_ELEMENT = blocked_ranges.length - 1;

			if (blocked_ranges.length == 0) {
				blocked_ranges.push(ip_list[LO]);

			} else if (change_higher_value(ip_pair, blocked_ranges[LAST_ELEMENT])) {
				var tmp = blocked_ranges.pop();
				tmp[HI] = ip_pair[HI];
				blocked_ranges.push(tmp);

			} else if (!contained(ip_pair, blocked_ranges[LAST_ELEMENT])) {
				blocked_ranges.push(ip_pair);
			}
		}

		return blocked_ranges.length;
	}

	static inline function change_higher_value(interval_1:Array<Int>, interval_2:Array<Int>):Bool {
		return interval_2[LO] < interval_1[LO] && interval_1[LO] < interval_2[HI] && interval_2[HI] <= interval_1[HI];
	}

	static inline function contained(interval_1:Array<Int>, interval_2:Array<Int>):Bool {
		return interval_2[LO] < interval_1[LO] && interval_1[HI] < interval_2[HI];
	}

	static function process_all_files():Void {
		Sys.println("Getting answers...");

		var answers:Map<String, Int> = new Map<String, Int>();
		for (_ => value in file_paths) {
			final ip_list = read_file(value);
			final file_name = value.split("/").pop();
			ip_list.sort(function(x:Array<Int>, y:Array<Int>) {
				return x[LO] - y[LO];
			});
			final length = least_possible_size(ip_list);
			answers[file_name] = length;
		}

		Sys.println("");
		for (key => value in answers) {
			Sys.println(">>> " + key + " => " + value);
		}
		Sys.println("");
	}
}
