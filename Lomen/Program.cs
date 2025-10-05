using System;
using System.Drawing;
using CommandLine;

namespace Lomen
{
    internal static class Program
    {
        private static void Main(string[] args)
        {
            if (args.Length == 0) args = new[] { "--help" };
            Parser.Default.ParseArguments<Options>(args).WithParsed(Run);
        }

        private static void Run(Options options)
        {
            if (options.RightColor != null ||
                options.CenterColor != null ||
                options.LeftColor != null ||
                options.GameColor != null ||
                options.AllColors != null)
                SetColors(options.RightColor, options.CenterColor, options.LeftColor, options.GameColor,
                    options.AllColors);

            if (options.Info) ShowStatus();
        }

        private static void ShowStatus()
        {
            Console.WriteLine($"Keyboard type: {KbdLightingControl.GetKeyboardType()}");
            Console.WriteLine($"Lighting supported: {(KbdLightingControl.IsLightingSupported() ? "Yes" : "No")}");

            var colors = KbdLightingControl.GetColors();
            Console.WriteLine("Lighting colors: "
                              + $"right={ColorTranslator.ToHtml(colors.Right)} "
                              + $"center={ColorTranslator.ToHtml(colors.Center)} "
                              + $"left={ColorTranslator.ToHtml(colors.Left)} "
                              + $"game={ColorTranslator.ToHtml(colors.Game)}");
        }

        private static void SetColors(string rightColor, string centerColor, string leftColor,
            string gameColor, string defaultColor)
        {
            var d = Util.ParseColor(defaultColor);
            var colors = new KbdZoneColors(
                Util.ParseColor(rightColor, d),
                Util.ParseColor(centerColor, d),
                Util.ParseColor(leftColor, d),
                Util.ParseColor(gameColor, d));
            KbdLightingControl.SetColors(colors);
        }

        // ReSharper disable once ClassNeverInstantiated.Local
        // ReSharper disable UnusedAutoPropertyAccessor.Local
        private class Options
        {
            [Option("info", HelpText = "Displays keyboard lighting status information.")]
            public bool Info { get; set; }

            [Option('1', "right",
                HelpText = "Sets color for the first zone of the keyboard. " +
                           "Use color name or hex code.")]
            public string RightColor { get; set; }

            [Option('2', "center",
                HelpText = "Sets color for the second zone of the keyboard. " +
                           "Use color name or hex code.")]
            public string CenterColor { get; set; }

            [Option('3', "left",
                HelpText = "Sets color for the third zone of the keyboard. " +
                           "Use color name or hex code.")]
            public string LeftColor { get; set; }

            [Option('4', "game",
                HelpText = "Sets color for the forth zone of the keyboard. " +
                           "Use color name or hex code.")]
            public string GameColor { get; set; }

            [Option('a', "all",
                HelpText = "Sets color for all zones of the keyboard except those specified specifically. " +
                           "Use color name or hex code.")]
            public string AllColors { get; set; }
        }
    }
}