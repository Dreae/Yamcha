#include <sourcemod>

public Plugin myinfo = {
    name = "Yamcha",
    author = "Dreae",
    description = "Adds extra events for Yamcha monitoring",
    version = "1.0",
    url = "https://github.com/Dreae/Yamcha"
};

int shots_fired[MAXPLAYERS + 1];
int shots_hit[MAXPLAYERS + 1];

public void OnPluginStart() {
    RegServerCmd("yamcha_notify", cmd_yamcha_notify, "Internal command for notifying players", FCVAR_NONE);
}

public void OnEntityCreated(int entity, const char[] classname) {
    if (0 < entity < MaxClients) {
        SDKHook(entity, SDKHook_TraceAttackPost, trace_attack_post);
    }
}

public void OnMapStart() {
    for (int i = 0; i < MAXPLAYERS + 1; i++) {
        shots_fired[i] = 0;
        shots_hit[i] = 0;
    }

    CreateTimer(10.0, log_shots, 0, TIMER_REPEAT | TIMER_FLAG_NO_MAPCHANGE);
}

// yamcha_notify <killer_id> <victim_id> <killer_points> <killer_delta> <victim_points> <victim_delta>
// [Yamcha] Dreae (2180) got 16 points for killing Brett (1992)[-12] 
public Action cmd_yamcha_notify(int args) {
    if (args != 6) {
        return Plugin_Handled;
    }
    
    int killer = GetClientOfUserId(get_cmd_arg_int(1));
    if (!is_client_valid(killer)) {
        return Plugin_Handled;
    }

    int victim = GetClientOfUserId(get_cmd_arg_int(2));
    if (!is_client_valid(victim)) {
        return Plugin_Handled;
    }

    int killer_points = get_cmd_arg_int(3);
    int killer_delta = get_cmd_arg_int(4);
    int victim_points = get_cmd_arg_int(5);
    int victim_delta = get_cmd_arg_int(6);

    PrintToChat(killer, "%T", "Yamcha", killer, killer_points, killer_delta, victim, victim_points, victim_delta);
    PrintToChat(victim, "%T", "Yamcha", killer, killer_points, killer_delta, victim, victim_points, victim_delta);
}

public void trace_attack_post(int victim, int attacker, int inflictor, float damage, int damage_type, int ammo_type, int hitbox, int hitgroup) {
    if (!(0 < attacker < MaxClients)) {
        return;
    }

    shots_fired[attacker]++;
    
    if (0 < victim < MaxClients) {
        shots_hit[attacker]++;
    }
}

Action log_shots(Handle timer) {
    for (int i = 1; i < MaxClients; i++) {
        if (IsClientConnected(i)) {
            LogToGame("accuracy_update %d shots_fired %d shots_hit %d", GetClientUserId(i), shots_fired[i], shots_hit[i]);
        }
    }
}

stock bool is_client_valid(int client_id) {
    return client_id != 0 && IsClientConnected(client_id) && IsClientInGame(client_id);
}

stock int get_cmd_arg_int(int arg_num) {
    char cmd_int[16];
    GetCmdArg(arg_num, cmd_int, sizeof(cmd_int));

    return StringToInt(cmd_int, 10);
}