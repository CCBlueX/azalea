// NOTE: This file is generated automatically by codegen/packet.py.
// Don't edit it directly!

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(GamePacket,
    Clientbound => [
        bundle_delimiter, // 0x00
        add_entity, // 0x01
        add_experience_orb, // 0x02
        animate, // 0x03
        award_stats, // 0x04
        block_changed_ack, // 0x05
        block_destruction, // 0x06
        block_entity_data, // 0x07
        block_event, // 0x08
        block_update, // 0x09
        boss_event, // 0x0A
        change_difficulty, // 0x0B
        chunk_batch_finished, // 0x0C
        chunk_batch_start, // 0x0D
        chunks_biomes, // 0x0E
        clear_titles, // 0x0F
        command_suggestions, // 0x10
        commands, // 0x11
        container_close, // 0x12
        container_set_content, // 0x13
        container_set_data, // 0x14
        container_set_slot, // 0x15
        cookie_request, // 0x16
        cooldown, // 0x17
        custom_chat_completions, // 0x18
        custom_payload, // 0x19
        damage_event, // 0x1A
        debug_sample, // 0x1B
        delete_chat, // 0x1C
        disconnect, // 0x1D
        disguised_chat, // 0x1E
        entity_event, // 0x1F
        entity_position_sync, // 0x20
        explode, // 0x21
        forget_level_chunk, // 0x22
        game_event, // 0x23
        horse_screen_open, // 0x24
        hurt_animation, // 0x25
        initialize_border, // 0x26
        keep_alive, // 0x27
        level_chunk_with_light, // 0x28
        level_event, // 0x29
        level_particles, // 0x2A
        light_update, // 0x2B
        login, // 0x2C
        map_item_data, // 0x2D
        merchant_offers, // 0x2E
        move_entity_pos, // 0x2F
        move_entity_pos_rot, // 0x30
        move_minecart_along_track, // 0x31
        move_entity_rot, // 0x32
        move_vehicle, // 0x33
        open_book, // 0x34
        open_screen, // 0x35
        open_sign_editor, // 0x36
        ping, // 0x37
        pong_response, // 0x38
        place_ghost_recipe, // 0x39
        player_abilities, // 0x3A
        player_chat, // 0x3B
        player_combat_end, // 0x3C
        player_combat_enter, // 0x3D
        player_combat_kill, // 0x3E
        player_info_remove, // 0x3F
        player_info_update, // 0x40
        player_look_at, // 0x41
        player_position, // 0x42
        player_rotation, // 0x43
        recipe_book_add, // 0x44
        recipe_book_remove, // 0x45
        recipe_book_settings, // 0x46
        remove_entities, // 0x47
        remove_mob_effect, // 0x48
        reset_score, // 0x49
        resource_pack_pop, // 0x4A
        resource_pack_push, // 0x4B
        respawn, // 0x4C
        rotate_head, // 0x4D
        section_blocks_update, // 0x4E
        select_advancements_tab, // 0x4F
        server_data, // 0x50
        set_action_bar_text, // 0x51
        set_border_center, // 0x52
        set_border_lerp_size, // 0x53
        set_border_size, // 0x54
        set_border_warning_delay, // 0x55
        set_border_warning_distance, // 0x56
        set_camera, // 0x57
        set_chunk_cache_center, // 0x58
        set_chunk_cache_radius, // 0x59
        set_cursor_item, // 0x5A
        set_default_spawn_position, // 0x5B
        set_display_objective, // 0x5C
        set_entity_data, // 0x5D
        set_entity_link, // 0x5E
        set_entity_motion, // 0x5F
        set_equipment, // 0x60
        set_experience, // 0x61
        set_health, // 0x62
        set_held_slot, // 0x63
        set_objective, // 0x64
        set_passengers, // 0x65
        set_player_inventory, // 0x66
        set_player_team, // 0x67
        set_score, // 0x68
        set_simulation_distance, // 0x69
        set_subtitle_text, // 0x6A
        set_time, // 0x6B
        set_title_text, // 0x6C
        set_titles_animation, // 0x6D
        sound_entity, // 0x6E
        sound, // 0x6F
        start_configuration, // 0x70
        stop_sound, // 0x71
        store_cookie, // 0x72
        system_chat, // 0x73
        tab_list, // 0x74
        tag_query, // 0x75
        take_item_entity, // 0x76
        teleport_entity, // 0x77
        ticking_state, // 0x78
        ticking_step, // 0x79
        transfer, // 0x7A
        update_advancements, // 0x7B
        update_attributes, // 0x7C
        update_mob_effect, // 0x7D
        update_recipes, // 0x7E
        update_tags, // 0x7F
        projectile_power, // 0x80
        custom_report_details, // 0x81
        server_links, // 0x82
    ],
    Serverbound => [
        accept_teleportation, // 0x00
        block_entity_tag_query, // 0x01
        bundle_item_selected, // 0x02
        change_difficulty, // 0x03
        chat_ack, // 0x04
        chat_command, // 0x05
        chat_command_signed, // 0x06
        chat, // 0x07
        chat_session_update, // 0x08
        chunk_batch_received, // 0x09
        client_command, // 0x0A
        client_tick_end, // 0x0B
        client_information, // 0x0C
        command_suggestion, // 0x0D
        configuration_acknowledged, // 0x0E
        container_button_click, // 0x0F
        container_click, // 0x10
        container_close, // 0x11
        container_slot_state_changed, // 0x12
        cookie_response, // 0x13
        custom_payload, // 0x14
        debug_sample_subscription, // 0x15
        edit_book, // 0x16
        entity_tag_query, // 0x17
        interact, // 0x18
        jigsaw_generate, // 0x19
        keep_alive, // 0x1A
        lock_difficulty, // 0x1B
        move_player_pos, // 0x1C
        move_player_pos_rot, // 0x1D
        move_player_rot, // 0x1E
        move_player_status_only, // 0x1F
        move_vehicle, // 0x20
        paddle_boat, // 0x21
        pick_item_from_block, // 0x22
        pick_item_from_entity, // 0x23
        ping_request, // 0x24
        place_recipe, // 0x25
        player_abilities, // 0x26
        player_action, // 0x27
        player_command, // 0x28
        player_input, // 0x29
        player_loaded, // 0x2A
        pong, // 0x2B
        recipe_book_change_settings, // 0x2C
        recipe_book_seen_recipe, // 0x2D
        rename_item, // 0x2E
        resource_pack, // 0x2F
        seen_advancements, // 0x30
        select_trade, // 0x31
        set_beacon, // 0x32
        set_carried_item, // 0x33
        set_command_block, // 0x34
        set_command_minecart, // 0x35
        set_creative_mode_slot, // 0x36
        set_jigsaw_block, // 0x37
        set_structure_block, // 0x38
        sign_update, // 0x39
        swing, // 0x3A
        teleport_to_entity, // 0x3B
        use_item_on, // 0x3C
        use_item, // 0x3D
    ]
);
