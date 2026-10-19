// Original minimal Openplanet plugin, CC0 / Unlicense.
// The two unknown names intentionally demonstrate real LSP diagnostics.
void Main() {
    print("Openplanet language tools are connected.");
    MissingType value;
    missing_function();
}

void RenderMenu() {
    if (UI::MenuItem("Zed smoke test")) {
        print("Hello from Openplanet");
    }
}
