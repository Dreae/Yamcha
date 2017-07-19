pub enum CSGO_Weapon {
    weapon_ak47
    weapon_aug
    weapon_awp
    weapon_bizon
    weapon_c4
    weapon_cz75a
    weapon_deagle
    weapon_decoy
    weapon_elite
    weapon_famas
    weapon_fiveseven
    weapon_flashbang
    weapon_g3sg1
    weapon_galilar
    weapon_glock
    weapon_healthshot
    weapon_hegrenade
    weapon_incgrenade
    weapon_hkp2000
    weapon_knife
    weapon_m249
    weapon_m4a1
    weapon_m4a1_silencer
    weapon_mac10
    weapon_mag7
    weapon_molotov
    weapon_mp7
    weapon_mp9
    weapon_negev
    weapon_nova
    weapon_p250
    weapon_p90
    weapon_sawedoff
    weapon_scar20
    weapon_sg556
    weapon_ssg08
    weapon_smokegrenade
    weapon_tagrenade
    weapon_taser
    weapon_tec9
    weapon_ump45
    weapon_usp_silencer
    weapon_xm1014
    weapon_revolver
}

pub struct ConnectedPlayerWeaponState {
    weapon: CSGO_Weapon,
    kills: i32,
    deaths: i32,
    headshots: i32,
    shots_fired: i64,
    shots_hit: i64,
}

impl ConnectedPlayerWeaponState {
    pub fn new(weapon: CSGO_Weapon) -> ConnectedPlayerWeaponState {
        ConnectedPlayerWeaponState {
            weapon: weapon,
            kills: 0,
            deaths: 0,
            headshots: 0,
            shots_fired: 0,
            shots_hit: 0,
        }
    }
}