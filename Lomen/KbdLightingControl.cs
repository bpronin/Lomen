using System;
using System.Drawing;
using System.Management;

namespace Lomen
{
    public enum KbdType
    {
        Normal,
        WithNumpad,
        WithoutNumpad,
        Rgb,
        OneZoneWithNumpad,
        OneZoneWithoutNumpad
    }

    public static class KbdLightingControl
    {
        private const int CmdCommon = 131081;
        private const int CmdGaming = 131080;

        private const int CmdTypeGetPlatformInfo = 1;
        private const int CmdTypeGetZoneColors = 2;
        private const int CmdTypeSetZoneColors = 3;
        private const int CmdTypeStatus = 4;
        private const int CmdTypeSetBrightness = 5;
        private const int CmdTypeSetLightBarColors = 11;
        private const int CmdTypeGetKeyboardType = 43;

        private const byte LightingLevelOn = 228;
        private const byte LightingLevelOff = 100;

        private const int RightZoneIndex = 0;
        private const int CenterZoneIndex = 1;
        private const int LeftZoneIndex = 2;
        private const int GameZoneIndex = 3;

        private const int ColorsDataOffset = 25;

        private static readonly byte[] Sign = { 83, 69, 67, 85 };

        private static void ExecuteCommand(
            int command,
            int commandType,
            byte[] inData,
            out byte[] outData)
        {
            const string wmiMethod = "hpqBIOSInt128";
            uint rwReturnCode;
            try
            {
                var obj = new ManagementObject(@"root\wmi", @"hpqBIntM.InstanceName='ACPI\PNP0C14\0_0'", null);

                var paramsData = new ManagementClass(@"root\wmi:hpqBDataIn");
                paramsData["Sign"] = Sign;
                paramsData["Command"] = command;
                paramsData["CommandType"] = commandType;
                paramsData["Size"] = inData?.Length ?? 0;
                paramsData["hpqBData"] = inData;

                var inParams = obj.GetMethodParameters(wmiMethod);
                inParams["InData"] = paramsData;

                var invokeOptions = new InvokeMethodOptions { Timeout = TimeSpan.MaxValue };
                var invokeResult = obj.InvokeMethod(wmiMethod, inParams, invokeOptions);

                var outParams = invokeResult?["OutData"] as ManagementBaseObject;
                outData = outParams?["Data"] as byte[];

                rwReturnCode = Convert.ToUInt32(outParams?["rwReturnCode"]);
            }
            catch (Exception ex)
            {
                throw new Exception("Error executing WMI command.", ex);
            }

            if (rwReturnCode != 0)
                throw new Exception($"Unexpected command execution (return code: {rwReturnCode}).");
        }

        private static byte[] GetColorsData()
        {
            ExecuteCommand(CmdCommon, CmdTypeGetZoneColors, null, out var result);
            return result;
        }

        private static Color GetZoneColorData(byte[] data, int zoneIndex)
        {
            var offset = ColorsDataOffset + zoneIndex * 3;
            return Color.FromArgb(data[offset], data[offset + 1], data[offset + 2]);
        }

        private static void SetZoneColorData(byte[] data, int zoneIndex, Color color)
        {
            if (color.IsEmpty) return;

            var offset = ColorsDataOffset + zoneIndex * 3;
            data[offset] = color.R;
            data[offset + 1] = color.G;
            data[offset + 2] = color.B;
        }

        public static KbdType GetKeyboardType()
        {
            ExecuteCommand(CmdGaming, CmdTypeGetKeyboardType, null, out var result);
            return (KbdType)result[0];
        }

        public static bool IsLightingSupported()
        {
            ExecuteCommand(CmdCommon, CmdTypeGetPlatformInfo, null, out var result);
            return (result[0] & 1) == 1;
        }

        public static bool IsLightBarSupported()
        {
            ExecuteCommand(CmdGaming, CmdTypeGetPlatformInfo, null, out var result);
            return ((result[0] >> 1) & 1) == 1;
        }

        public static bool IsLightingOn()
        {
            ExecuteCommand(CmdCommon, CmdTypeStatus, null, out var result);
            switch (result[0])
            {
                case LightingLevelOn:
                    return true;
                case LightingLevelOff:
                    return false;
                default:
                    throw new Exception($"Unexpected result data: {result[0]} .");
            }
        }

        public static void SetLightingOn(bool enabled)
        {
            var value = enabled ? LightingLevelOn : LightingLevelOff;
            ExecuteCommand(CmdCommon, CmdTypeSetBrightness, new byte[] { value, 0, 0, 0 }, out _);
        }

        public static KbdZoneColors GetColors()
        {
            var data = GetColorsData();
            return new KbdZoneColors(
                GetZoneColorData(data, RightZoneIndex),
                GetZoneColorData(data, CenterZoneIndex),
                GetZoneColorData(data, LeftZoneIndex),
                GetZoneColorData(data, GameZoneIndex)
            );
        }

        public static void SetColors(KbdZoneColors colors)
        {
            var data = GetColorsData();
            SetZoneColorData(data, RightZoneIndex, colors.Right);
            SetZoneColorData(data, CenterZoneIndex, colors.Center);
            SetZoneColorData(data, LeftZoneIndex, colors.Left);
            SetZoneColorData(data, GameZoneIndex, colors.Game);

            ExecuteCommand(CmdCommon, CmdTypeSetZoneColors, data, out _);
        }

        public static void SetLightBarColors(Color[] colors)
        {
            var data = new byte[128];
            data[1] = 0; /* MODE */
            data[3] = 100; /* BRIGHTNESS */
            data[6] = 4; /* COLOR_COUNT */

            data[7] = colors[0].R; /* ZONE1_RGB_R */
            data[8] = colors[0].G; /* ZONE1_RGB_G */
            data[9] = colors[0].B; /* ZONE1_RGB_B */

            data[10] = colors[1].R; /* ZONE2_RGB_R */
            data[11] = colors[1].G; /* ZONE2_RGB_G */
            data[12] = colors[1].B; /* ZONE2_RGB_B */

            data[13] = colors[2].R; /* ZONE3_RGB_G */
            data[14] = colors[2].G; /* ZONE3_RGB_R */
            data[15] = colors[2].B; /* ZONE3_RGB_B */

            data[16] = colors[3].R; /* ZONE4_RGB_G */
            data[17] = colors[3].G; /* ZONE4_RGB_R */
            data[18] = colors[3].B; /* ZONE4_RGB_B */

            ExecuteCommand(CmdCommon, CmdTypeSetLightBarColors, data, out _);
        }
    }
}