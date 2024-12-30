pub const HID_KEY_STRINGS: [&str; 169] = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "NUM_1", "NUM_2", "NUM_3", "NUM_4", "NUM_5", "NUM_6", "NUM_7", "NUM_8", "NUM_9", "NUM_0", "ENTER", "ESCAPE", "BACKSPACE", "TAB", "SPACE", "MINUS", "EQUAL", "BRACKET_LEFT", "BRACKET_RIGHT", "BACKSLASH", "EUROPE_1", "SEMICOLON", "APOSTROPHE", "GRAVE", "COMMA", "PERIOD", "SLASH", "CAPS_LOCK", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12", "PRINT_SCREEN", "SCROLL_LOCK", "PAUSE", "INSERT", "HOME", "PAGE_UP", "DELETE", "END", "PAGE_DOWN", "ARROW_RIGHT", "ARROW_LEFT", "ARROW_DOWN", "ARROW_UP", "NUM_LOCK", "KEYPAD_DIVIDE", "KEYPAD_MULTIPLY", "KEYPAD_SUBTRACT", "KEYPAD_ADD", "KEYPAD_ENTER", "KEYPAD_1", "KEYPAD_2", "KEYPAD_3", "KEYPAD_4", "KEYPAD_5", "KEYPAD_6", "KEYPAD_7", "KEYPAD_8", "KEYPAD_9", "KEYPAD_0", "KEYPAD_DECIMAL", "EUROPE_2", "APPLICATION", "POWER", "KEYPAD_EQUAL", "F13", "F14", "F15", "F16", "F17", "F18", "F19", "F20", "F21", "F22", "F23", "F24", "EXECUTE", "HELP", "MENU", "SELECT", "STOP", "AGAIN", "UNDO", "CUT", "COPY", "PASTE", "FIND", "MUTE", "VOLUME_UP", "VOLUME_DOWN", "LOCKING_CAPS_LOCK", "LOCKING_NUM_LOCK", "LOCKING_SCROLL_LOCK", "KEYPAD_COMMA", "KEYPAD_EQUAL_SIGN", "KANJI1", "KANJI2", "KANJI3", "KANJI4", "KANJI5", "KANJI6", "KANJI7", "KANJI8", "KANJI9", "LANG1", "LANG2", "LANG3", "LANG4", "LANG5", "LANG6", "LANG7", "LANG8", "LANG9", "ALTERNATE_ERASE", "SYSREQ_ATTENTION", "CANCEL", "CLEAR", "PRIOR", "RETURN", "SEPARATOR", "OUT", "OPER", "CLEAR_AGAIN", "CRSEL_PROPS", "EXSEL", "CTRL", "SHIFT", "ALT", "GUI", "CTRL_RIGHT", "SHIFT_RIGHT", "ALT_RIGHT", "GUI_RIGHT"];
pub const HID_KEY_CODES: [u8; 169] = [0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7A, 0x7B, 0x7C, 0x7D, 0x7E, 0x7F, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0x8F, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0x9B, 0x9C, 0x9D, 0x9E, 0x9F, 0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xE0, 0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7];

pub const MOUSE_BUTTON_STRINGS: [&str; 3] = ["LEFT_CLICK", "RIGHT_CLICK", "MIDDLE_CLICK"];
pub const MOUSE_BUTTON_CODES: [u8; 3] = [0x01, 0x02, 0x04];

pub fn hid_string_to_code(name: &str) -> Option<u8> {
    for i in 0..HID_KEY_STRINGS.len() {
        if HID_KEY_STRINGS[i] == name {
            return Some(HID_KEY_CODES[i]);
        }
    }

    None
}

pub fn hid_code_to_string(codes: &Vec<u8>) -> Option<String> {
    let mut result = String::from("");
    let mut idx = 0;
    while idx < codes.len() {
        let mut found = false;
        for i in 0..HID_KEY_CODES.len() {
            if HID_KEY_CODES[i] == codes[idx] {
                if result == "" {
                    result += HID_KEY_STRINGS[i];
                } else {
                    result += format!(" + {}", HID_KEY_STRINGS[i]).as_str();
                }

                found = true;
            }
        }

        if !found {
            return None;
        } else {
            idx += 1;
        }
    }

    Some(result)
}

pub fn mouse_string_to_code(name: &str) -> Option<u8> {
    for i in 0..MOUSE_BUTTON_STRINGS.len() {
        if MOUSE_BUTTON_STRINGS[i] == name {
            return Some(MOUSE_BUTTON_CODES[i]);
        }
    }

    None
}

pub fn mouse_code_to_string(code: u8) -> Option<String> {
    for i in 0..MOUSE_BUTTON_CODES.len() {
        if MOUSE_BUTTON_CODES[i] == code {
            return Some(MOUSE_BUTTON_STRINGS[i].to_string());
        }
    }

    return None;
}

pub fn hid_key_sequence_to_string(seq: &Vec<Vec<u8>>) -> Option<String> {
    let mut result = String::from("");
    for codes in seq {
        let hid = hid_code_to_string(codes)?;
        if result == "" {
            result += hid.as_str();
        } else {
            result += format!(" | {}", hid).as_str();
        }
    }

    Some(result)
}


// HID Key Codes
pub const HID_LEFT_MB: u8 = 0x01;
pub const HID_RIGHT_MB: u8 = 0x02;
pub const HID_MIDDLE_MB: u8 = 0x04;
pub const HID_KEY_NONE: u8 = 0x00;
pub const HID_KEY_A: u8 = 0x04;
pub const HID_KEY_B: u8 = 0x05;
pub const HID_KEY_C: u8 = 0x06;
pub const HID_KEY_D: u8 = 0x07;
pub const HID_KEY_E: u8 = 0x08;
pub const HID_KEY_F: u8 = 0x09;
pub const HID_KEY_G: u8 = 0x0A;
pub const HID_KEY_H: u8 = 0x0B;
pub const HID_KEY_I: u8 = 0x0C;
pub const HID_KEY_J: u8 = 0x0D;
pub const HID_KEY_K: u8 = 0x0E;
pub const HID_KEY_L: u8 = 0x0F;
pub const HID_KEY_M: u8 = 0x10;
pub const HID_KEY_N: u8 = 0x11;
pub const HID_KEY_O: u8 = 0x12;
pub const HID_KEY_P: u8 = 0x13;
pub const HID_KEY_Q: u8 = 0x14;
pub const HID_KEY_R: u8 = 0x15;
pub const HID_KEY_S: u8 = 0x16;
pub const HID_KEY_T: u8 = 0x17;
pub const HID_KEY_U: u8 = 0x18;
pub const HID_KEY_V: u8 = 0x19;
pub const HID_KEY_W: u8 = 0x1A;
pub const HID_KEY_X: u8 = 0x1B;
pub const HID_KEY_Y: u8 = 0x1C;
pub const HID_KEY_Z: u8 = 0x1D;
pub const HID_KEY_1: u8 = 0x1E;
pub const HID_KEY_2: u8 = 0x1F;
pub const HID_KEY_3: u8 = 0x20;
pub const HID_KEY_4: u8 = 0x21;
pub const HID_KEY_5: u8 = 0x22;
pub const HID_KEY_6: u8 = 0x23;
pub const HID_KEY_7: u8 = 0x24;
pub const HID_KEY_8: u8 = 0x25;
pub const HID_KEY_9: u8 = 0x26;
pub const HID_KEY_0: u8 = 0x27;
pub const HID_KEY_ENTER: u8 = 0x28;
pub const HID_KEY_ESCAPE: u8 = 0x29;
pub const HID_KEY_BACKSPACE: u8 = 0x2A;
pub const HID_KEY_TAB: u8 = 0x2B;
pub const HID_KEY_SPACE: u8 = 0x2C;
pub const HID_KEY_MINUS: u8 = 0x2D;
pub const HID_KEY_EQUAL: u8 = 0x2E;
pub const HID_KEY_BRACKET_LEFT: u8 = 0x2F;
pub const HID_KEY_BRACKET_RIGHT: u8 = 0x30;
pub const HID_KEY_BACKSLASH: u8 = 0x31;
pub const HID_KEY_EUROPE_1: u8 = 0x32;
pub const HID_KEY_SEMICOLON: u8 = 0x33;
pub const HID_KEY_APOSTROPHE: u8 = 0x34;
pub const HID_KEY_GRAVE: u8 = 0x35;
pub const HID_KEY_COMMA: u8 = 0x36;
pub const HID_KEY_PERIOD: u8 = 0x37;
pub const HID_KEY_SLASH: u8 = 0x38;
pub const HID_KEY_CAPS_LOCK: u8 = 0x39;
pub const HID_KEY_F1: u8 = 0x3A;
pub const HID_KEY_F2: u8 = 0x3B;
pub const HID_KEY_F3: u8 = 0x3C;
pub const HID_KEY_F4: u8 = 0x3D;
pub const HID_KEY_F5: u8 = 0x3E;
pub const HID_KEY_F6: u8 = 0x3F;
pub const HID_KEY_F7: u8 = 0x40;
pub const HID_KEY_F8: u8 = 0x41;
pub const HID_KEY_F9: u8 = 0x42;
pub const HID_KEY_F10: u8 = 0x43;
pub const HID_KEY_F11: u8 = 0x44;
pub const HID_KEY_F12: u8 = 0x45;
pub const HID_KEY_PRINT_SCREEN: u8 = 0x46;
pub const HID_KEY_SCROLL_LOCK: u8 = 0x47;
pub const HID_KEY_PAUSE: u8 = 0x48;
pub const HID_KEY_INSERT: u8 = 0x49;
pub const HID_KEY_HOME: u8 = 0x4A;
pub const HID_KEY_PAGE_UP: u8 = 0x4B;
pub const HID_KEY_DELETE: u8 = 0x4C;
pub const HID_KEY_END: u8 = 0x4D;
pub const HID_KEY_PAGE_DOWN: u8 = 0x4E;
pub const HID_KEY_ARROW_RIGHT: u8 = 0x4F;
pub const HID_KEY_ARROW_LEFT: u8 = 0x50;
pub const HID_KEY_ARROW_DOWN: u8 = 0x51;
pub const HID_KEY_ARROW_UP: u8 = 0x52;
pub const HID_KEY_NUM_LOCK: u8 = 0x53;
pub const HID_KEY_KEYPAD_DIVIDE: u8 = 0x54;
pub const HID_KEY_KEYPAD_MULTIPLY: u8 = 0x55;
pub const HID_KEY_KEYPAD_SUBTRACT: u8 = 0x56;
pub const HID_KEY_KEYPAD_ADD: u8 = 0x57;
pub const HID_KEY_KEYPAD_ENTER: u8 = 0x58;
pub const HID_KEY_KEYPAD_1: u8 = 0x59;
pub const HID_KEY_KEYPAD_2: u8 = 0x5A;
pub const HID_KEY_KEYPAD_3: u8 = 0x5B;
pub const HID_KEY_KEYPAD_4: u8 = 0x5C;
pub const HID_KEY_KEYPAD_5: u8 = 0x5D;
pub const HID_KEY_KEYPAD_6: u8 = 0x5E;
pub const HID_KEY_KEYPAD_7: u8 = 0x5F;
pub const HID_KEY_KEYPAD_8: u8 = 0x60;
pub const HID_KEY_KEYPAD_9: u8 = 0x61;
pub const HID_KEY_KEYPAD_0: u8 = 0x62;
pub const HID_KEY_KEYPAD_DECIMAL: u8 = 0x63;
pub const HID_KEY_EUROPE_2: u8 = 0x64;
pub const HID_KEY_APPLICATION: u8 = 0x65;
pub const HID_KEY_POWER: u8 = 0x66;
pub const HID_KEY_KEYPAD_EQUAL: u8 = 0x67;
pub const HID_KEY_F13: u8 = 0x68;
pub const HID_KEY_F14: u8 = 0x69;
pub const HID_KEY_F15: u8 = 0x6A;
pub const HID_KEY_F16: u8 = 0x6B;
pub const HID_KEY_F17: u8 = 0x6C;
pub const HID_KEY_F18: u8 = 0x6D;
pub const HID_KEY_F19: u8 = 0x6E;
pub const HID_KEY_F20: u8 = 0x6F;
pub const HID_KEY_F21: u8 = 0x70;
pub const HID_KEY_F22: u8 = 0x71;
pub const HID_KEY_F23: u8 = 0x72;
pub const HID_KEY_F24: u8 = 0x73;
pub const HID_KEY_EXECUTE: u8 = 0x74;
pub const HID_KEY_HELP: u8 = 0x75;
pub const HID_KEY_MENU: u8 = 0x76;
pub const HID_KEY_SELECT: u8 = 0x77;
pub const HID_KEY_STOP: u8 = 0x78;
pub const HID_KEY_AGAIN: u8 = 0x79;
pub const HID_KEY_UNDO: u8 = 0x7A;
pub const HID_KEY_CUT: u8 = 0x7B;
pub const HID_KEY_COPY: u8 = 0x7C;
pub const HID_KEY_PASTE: u8 = 0x7D;
pub const HID_KEY_FIND: u8 = 0x7E;
pub const HID_KEY_MUTE: u8 = 0x7F;
pub const HID_KEY_VOLUME_UP: u8 = 0x80;
pub const HID_KEY_VOLUME_DOWN: u8 = 0x81;
pub const HID_KEY_LOCKING_CAPS_LOCK: u8 = 0x82;
pub const HID_KEY_LOCKING_NUM_LOCK: u8 = 0x83;
pub const HID_KEY_LOCKING_SCROLL_LOCK: u8 = 0x84;
pub const HID_KEY_KEYPAD_COMMA: u8 = 0x85;
pub const HID_KEY_KEYPAD_EQUAL_SIGN: u8 = 0x86;
pub const HID_KEY_KANJI1: u8 = 0x87;
pub const HID_KEY_KANJI2: u8 = 0x88;
pub const HID_KEY_KANJI3: u8 = 0x89;
pub const HID_KEY_KANJI4: u8 = 0x8A;
pub const HID_KEY_KANJI5: u8 = 0x8B;
pub const HID_KEY_KANJI6: u8 = 0x8C;
pub const HID_KEY_KANJI7: u8 = 0x8D;
pub const HID_KEY_KANJI8: u8 = 0x8E;
pub const HID_KEY_KANJI9: u8 = 0x8F;
pub const HID_KEY_LANG1: u8 = 0x90;
pub const HID_KEY_LANG2: u8 = 0x91;
pub const HID_KEY_LANG3: u8 = 0x92;
pub const HID_KEY_LANG4: u8 = 0x93;
pub const HID_KEY_LANG5: u8 = 0x94;
pub const HID_KEY_LANG6: u8 = 0x95;
pub const HID_KEY_LANG7: u8 = 0x96;
pub const HID_KEY_LANG8: u8 = 0x97;
pub const HID_KEY_LANG9: u8 = 0x98;
pub const HID_KEY_ALTERNATE_ERASE: u8 = 0x99;
pub const HID_KEY_SYSREQ_ATTENTION: u8 = 0x9A;
pub const HID_KEY_CANCEL: u8 = 0x9B;
pub const HID_KEY_CLEAR: u8 = 0x9C;
pub const HID_KEY_PRIOR: u8 = 0x9D;
pub const HID_KEY_RETURN: u8 = 0x9E;
pub const HID_KEY_SEPARATOR: u8 = 0x9F;
pub const HID_KEY_OUT: u8 = 0xA0;
pub const HID_KEY_OPER: u8 = 0xA1;
pub const HID_KEY_CLEAR_AGAIN: u8 = 0xA2;
pub const HID_KEY_CRSEL_PROPS: u8 = 0xA3;
pub const HID_KEY_EXSEL: u8 = 0xA4;
pub const HID_KEY_CONTROL_LEFT: u8 = 0xE0;
pub const HID_KEY_SHIFT_LEFT: u8 = 0xE1;
pub const HID_KEY_ALT_LEFT: u8 = 0xE2;
pub const HID_KEY_GUI_LEFT: u8 = 0xE3;
pub const HID_KEY_CONTROL_RIGHT: u8 = 0xE4;
pub const HID_KEY_SHIFT_RIGHT: u8 = 0xE5;
pub const HID_KEY_ALT_RIGHT: u8 = 0xE6;
pub const HID_KEY_GUI_RIGHT: u8 = 0xE7;

#[rustfmt::skip]
pub const ASCII_2_HID: [[u8; 2]; 128] = [
    // 0-31 are invisible control codes, except 0x09 HT(TAB) and 0x0A LF(Enter)
    [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],
    [0, 0], [HID_KEY_TAB, 0], [HID_KEY_ENTER, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],
    [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],
    [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],
    [HID_KEY_SPACE, 0],                         // 32 ' '
    [HID_KEY_1, HID_KEY_SHIFT_LEFT],            // 33 '!'
    [HID_KEY_APOSTROPHE, HID_KEY_SHIFT_LEFT],   // 34 '"'
    [HID_KEY_3, HID_KEY_SHIFT_LEFT],            // 35 '#'
    [HID_KEY_4, HID_KEY_SHIFT_LEFT],            // 36 '$'
    [HID_KEY_5, HID_KEY_SHIFT_LEFT],            // 37 '%'
    [HID_KEY_7, HID_KEY_SHIFT_LEFT],            // 38 '&'
    [HID_KEY_APOSTROPHE, 0],                    // 39 '''
    [HID_KEY_9, HID_KEY_SHIFT_LEFT],            // 40 '('
    [HID_KEY_0, HID_KEY_SHIFT_LEFT],            // 41 ')'
    [HID_KEY_8, HID_KEY_SHIFT_LEFT],            // 42 '*'
    [HID_KEY_EQUAL, HID_KEY_SHIFT_LEFT],        // 43 '+'
    [HID_KEY_COMMA, 0],                         // 44 ','
    [HID_KEY_MINUS, 0],                         // 45 '-'
    [HID_KEY_PERIOD, 0],                        // 46 '.'
    [HID_KEY_SLASH, 0],                         // 47 '/'
    [HID_KEY_0, 0],                             // 48 '0'
    [HID_KEY_1, 0],                             // 49 '1'
    [HID_KEY_2, 0],                             // 50 '2'
    [HID_KEY_3, 0],                             // 51 '3'
    [HID_KEY_4, 0],                             // 52 '4'
    [HID_KEY_5, 0],                             // 53 '5'
    [HID_KEY_6, 0],                             // 54 '6'
    [HID_KEY_7, 0],                             // 55 '7'
    [HID_KEY_8, 0],                             // 56 '8'
    [HID_KEY_9, 0],                             // 57 '9'
    [HID_KEY_SEMICOLON, HID_KEY_SHIFT_LEFT],    // 58 ':'
    [HID_KEY_SEMICOLON, 0],                     // 59 ';'
    [HID_KEY_COMMA, HID_KEY_SHIFT_LEFT],        // 60 '<'
    [HID_KEY_EQUAL, 0],                         // 61 '='
    [HID_KEY_PERIOD, HID_KEY_SHIFT_LEFT],       // 62 '>'
    [HID_KEY_SLASH, HID_KEY_SHIFT_LEFT],        // 63 '?'
    [HID_KEY_2, HID_KEY_SHIFT_LEFT],            // 64 '@'
    [HID_KEY_A, HID_KEY_SHIFT_LEFT],            // 65 'A'
    [HID_KEY_B, HID_KEY_SHIFT_LEFT],            // 66 'B'
    [HID_KEY_C, HID_KEY_SHIFT_LEFT],            // 67 'C'
    [HID_KEY_D, HID_KEY_SHIFT_LEFT],            // 68 'D'
    [HID_KEY_E, HID_KEY_SHIFT_LEFT],            // 69 'E'
    [HID_KEY_F, HID_KEY_SHIFT_LEFT],            // 70 'F'
    [HID_KEY_G, HID_KEY_SHIFT_LEFT],            // 71 'G'
    [HID_KEY_H, HID_KEY_SHIFT_LEFT],            // 72 'H'
    [HID_KEY_I, HID_KEY_SHIFT_LEFT],            // 73 'I'
    [HID_KEY_J, HID_KEY_SHIFT_LEFT],            // 74 'J'
    [HID_KEY_K, HID_KEY_SHIFT_LEFT],            // 75 'K'
    [HID_KEY_L, HID_KEY_SHIFT_LEFT],            // 76 'L'
    [HID_KEY_M, HID_KEY_SHIFT_LEFT],            // 77 'M'
    [HID_KEY_N, HID_KEY_SHIFT_LEFT],            // 78 'N'
    [HID_KEY_O, HID_KEY_SHIFT_LEFT],            // 79 'O'
    [HID_KEY_P, HID_KEY_SHIFT_LEFT],            // 80 'P'
    [HID_KEY_Q, HID_KEY_SHIFT_LEFT],            // 81 'Q'
    [HID_KEY_R, HID_KEY_SHIFT_LEFT],            // 82 'R'
    [HID_KEY_S, HID_KEY_SHIFT_LEFT],            // 83 'S'
    [HID_KEY_T, HID_KEY_SHIFT_LEFT],            // 84 'T'
    [HID_KEY_U, HID_KEY_SHIFT_LEFT],            // 85 'U'
    [HID_KEY_V, HID_KEY_SHIFT_LEFT],            // 86 'V'
    [HID_KEY_W, HID_KEY_SHIFT_LEFT],            // 87 'W'
    [HID_KEY_X, HID_KEY_SHIFT_LEFT],            // 88 'X'
    [HID_KEY_Y, HID_KEY_SHIFT_LEFT],            // 89 'Y'
    [HID_KEY_Z, HID_KEY_SHIFT_LEFT],            // 90 'Z'
    [HID_KEY_BRACKET_LEFT, 0],                  // 91 '['
    [HID_KEY_BACKSLASH, 0],                     // 92 '\'
    [HID_KEY_BRACKET_RIGHT, 0],                 // 93 ']'
    [HID_KEY_6, HID_KEY_SHIFT_LEFT],            // 94 '^'
    [HID_KEY_MINUS, HID_KEY_SHIFT_LEFT],        // 95 '_'
    [HID_KEY_GRAVE, 0],                         // 96 '`'
    [HID_KEY_A, 0],                             // 97 'a'
    [HID_KEY_B, 0],                             // 98 'b'
    [HID_KEY_C, 0],                             // 99 'c'
    [HID_KEY_D, 0],                             // 100 'd'
    [HID_KEY_E, 0],                             // 101 'e'
    [HID_KEY_F, 0],                             // 102 'f'
    [HID_KEY_G, 0],                             // 103 'g'
    [HID_KEY_H, 0],                             // 104 'h'
    [HID_KEY_I, 0],                             // 105 'i'
    [HID_KEY_J, 0],                             // 106 'j'
    [HID_KEY_K, 0],                             // 107 'k'
    [HID_KEY_L, 0],                             // 108 'l'
    [HID_KEY_M, 0],                             // 109 'm'
    [HID_KEY_N, 0],                             // 110 'n'
    [HID_KEY_O, 0],                             // 111 'o'
    [HID_KEY_P, 0],                             // 112 'p'
    [HID_KEY_Q, 0],                             // 113 'q'
    [HID_KEY_R, 0],                             // 114 'r'
    [HID_KEY_S, 0],                             // 115 's'
    [HID_KEY_T, 0],                             // 116 't'
    [HID_KEY_U, 0],                             // 117 'u'
    [HID_KEY_V, 0],                             // 118 'v'
    [HID_KEY_W, 0],                             // 119 'w'
    [HID_KEY_X, 0],                             // 120 'x'
    [HID_KEY_Y, 0],                             // 121 'y'
    [HID_KEY_Z, 0],                             // 122 'z'
    [HID_KEY_BRACKET_LEFT, HID_KEY_SHIFT_LEFT], // 123 '{'
    [HID_KEY_BACKSLASH, HID_KEY_SHIFT_LEFT],    // 124 '|'
    [HID_KEY_BRACKET_RIGHT, HID_KEY_SHIFT_LEFT],// 125 '}'
    [HID_KEY_GRAVE, HID_KEY_SHIFT_LEFT],        // 126 '~'
    [0, 0],                                     // 127
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    None,
    Key(u8),
    Consumer(u16),
}