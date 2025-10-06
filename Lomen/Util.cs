using System.Drawing;

namespace Lomen
{
    public static class Util
    {
        public static Color ParseColor(string s, Color? defaultColor = null)
        {
            if (s == null) return defaultColor ?? Color.Empty;
            var color = Color.FromName(s);
            return color.IsKnownColor
                ? color
                : ColorTranslator.FromHtml(s[0] == '#' ? s : "#" + s);
        }
    }
}