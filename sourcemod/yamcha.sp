#include <sourcemod>

public Plugin myinfo = {
    name = "Yamcha",
    author = "Dreae",
    description = "Adds extra events for Yamcha monitoring",
    version = "1.0",
    url = "https://github.com/Dreae/Yamcha"
};

public void OnPluginStart() {
    RegServerCmd("yamcha_notify", cmd_yamcha_notify, "Internal command for notifying players", FCVAR_NONE);
}

// yamcha_notify <killer_id> <victim_id> <killer_points> <killer_delta> <victim_points> <victim_delta>
public Action cmd_yamcha_notify(int args) {
    if (args != 6) {
        return Plugin_Handled;
    }
    
    
}