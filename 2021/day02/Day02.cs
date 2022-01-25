using System.IO;
using System.Linq;
using static System.Console;

namespace adventOfCode.Days
{
    class Day02
    {
        public static void Main()
        {
            // Test Part one against example and find number
            WriteLine(PartOne("testInput.txt"));
            WriteLine(PartOne("input.txt"));
            // Test Part Two against example and find number
            WriteLine(PartTwo("testInput.txt"));
            WriteLine(PartTwo("input.txt"));

        }

        public static int PartOne(string file)
        {
            // Part one
            var lines = File.ReadLines(file).ToList();

            var horizontalPos = 0;
            var depth = 0;

            foreach (string line in lines)
            {
                var v = line.Split(" ");

                switch (v[0])
                {
                    case "forward":
                        horizontalPos = horizontalPos + int.Parse(v[1]);
                        break;
                    case "up":
                        depth = depth - int.Parse(v[1]);
                        break;
                    default:
                        depth = depth + int.Parse(v[1]);
                        break;
                }
            }
            int result = horizontalPos * depth;
            return result;
        }

        public static int PartTwo(string file)
        {
            var lines = File.ReadLines(file).ToList();

            var horizontalPos = 0;
            var depth = 0;
            var aim = 0;

            foreach (string line in lines)
            {
                var v = line.Split(" ");

                switch (v[0])
                {
                    case "forward":
                        horizontalPos += int.Parse(v[1]);
                        depth += aim * int.Parse(v[1]);
                        break;
                    case "up":
                        aim -= int.Parse(v[1]);
                        break;
                    default:
                        aim += int.Parse(v[1]);
                        break;
                }
            }

            int result = horizontalPos * depth;
            return result;
        }
    }
}
