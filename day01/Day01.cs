using System.IO;
using System.Linq;
using static System.Console;

namespace adventOfCode.Days
{
    class Day01
    {
        public static void Main()
        {
            // Part one
            var lines = File.ReadLines("input.txt").ToList(); // add all numbers to a list
            var measurements = lines.Select(int.Parse).ToList(); // parse all numbers as int to a new list

            var lastMeasurement = int.MaxValue;
            var incOne = 0;

            // Trick here is that it checks first time, sees that measurement
            // is smaller than lastmeasurement, then sets the first value as
            // lastmeasurement and runs through the whole list.
            foreach (var measurement in measurements) {
                if (measurement > lastMeasurement) incOne++;
                lastMeasurement = measurement;
            }
            WriteLine($"Part One result: {incOne}");

            // Par two
            var lastSum = int.MaxValue;
            var incTwo = 0;
            for (var i = 0; i < measurements.Count - 2; i++)
            {
                var sum = measurements[i] + measurements[i + 1] + measurements[i + 2];
                if (sum > lastSum) incTwo++;
                lastSum = sum;
            }
            WriteLine($"Part Two result: {incTwo}");
        }
    }
}
