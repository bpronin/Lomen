using System.Drawing;

namespace Lomen
{
    public class KbdZoneColors
    {
        public readonly Color Center;
        public readonly Color Game;
        public readonly Color Left;
        public readonly Color Right;

        public KbdZoneColors(Color right, Color center, Color left, Color game)
        {
            Right = right;
            Center = center;
            Left = left;
            Game = game;
        }

        public override string ToString()
        {
            return $"{base.ToString()} {{left={Left}, right={Right}, center={Center}, game={Game}}}";
        }
    }
}