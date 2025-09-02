#include "styles.h"
#include "images.h"
#include "fonts.h"

#include "ui.h"
#include "screens.h"

//
// Style: tab_panel
//

void init_style_tab_panel_MAIN_DEFAULT(lv_style_t *style) {
    lv_style_set_radius(style, 0);
    lv_style_set_pad_top(style, 0);
    lv_style_set_pad_bottom(style, 0);
    lv_style_set_pad_left(style, 0);
    lv_style_set_pad_right(style, 0);
    lv_style_set_border_width(style, 0);
    lv_style_set_bg_color(style, lv_color_hex(0xfff2f6b1));
};

lv_style_t *get_style_tab_panel_MAIN_DEFAULT() {
    static lv_style_t *style;
    if (!style) {
        style = lv_mem_alloc(sizeof(lv_style_t));
        lv_style_init(style);
        init_style_tab_panel_MAIN_DEFAULT(style);
    }
    return style;
};

void add_style_tab_panel(lv_obj_t *obj) {
    (void)obj;
    lv_obj_add_style(obj, get_style_tab_panel_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

void remove_style_tab_panel(lv_obj_t *obj) {
    (void)obj;
    lv_obj_remove_style(obj, get_style_tab_panel_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

//
// Style: tab_button
//

void init_style_tab_button_MAIN_DEFAULT(lv_style_t *style) {
    lv_style_set_radius(style, 0);
    lv_style_set_bg_color(style, lv_color_hex(0xfff2f6b1));
    lv_style_set_text_color(style, lv_color_hex(0xff000000));
    lv_style_set_border_color(style, lv_color_hex(0xffffffff));
    lv_style_set_border_width(style, 2);
    lv_style_set_border_side(style, LV_BORDER_SIDE_LEFT);
};

lv_style_t *get_style_tab_button_MAIN_DEFAULT() {
    static lv_style_t *style;
    if (!style) {
        style = lv_mem_alloc(sizeof(lv_style_t));
        lv_style_init(style);
        init_style_tab_button_MAIN_DEFAULT(style);
    }
    return style;
};

void add_style_tab_button(lv_obj_t *obj) {
    (void)obj;
    lv_obj_add_style(obj, get_style_tab_button_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

void remove_style_tab_button(lv_obj_t *obj) {
    (void)obj;
    lv_obj_remove_style(obj, get_style_tab_button_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

//
// Style: tab_active
//

void init_style_tab_active_MAIN_DEFAULT(lv_style_t *style) {
    lv_style_set_bg_color(style, lv_color_hex(0xfff5f5f5));
    lv_style_set_bg_opa(style, 255);
    lv_style_set_pad_top(style, 11);
    lv_style_set_pad_bottom(style, 11);
    lv_style_set_pad_left(style, 20);
    lv_style_set_pad_right(style, 20);
    lv_style_set_border_color(style, lv_color_hex(0xff000000));
    lv_style_set_border_width(style, 1);
    lv_style_set_border_side(style, LV_BORDER_SIDE_TOP|LV_BORDER_SIDE_LEFT);
};

lv_style_t *get_style_tab_active_MAIN_DEFAULT() {
    static lv_style_t *style;
    if (!style) {
        style = lv_mem_alloc(sizeof(lv_style_t));
        lv_style_init(style);
        init_style_tab_active_MAIN_DEFAULT(style);
    }
    return style;
};

void add_style_tab_active(lv_obj_t *obj) {
    (void)obj;
    lv_obj_add_style(obj, get_style_tab_active_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

void remove_style_tab_active(lv_obj_t *obj) {
    (void)obj;
    lv_obj_remove_style(obj, get_style_tab_active_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

//
// Style: hot_temp
//

void init_style_hot_temp_MAIN_DEFAULT(lv_style_t *style) {
    lv_style_set_text_color(style, lv_color_hex(0xffff0000));
};

lv_style_t *get_style_hot_temp_MAIN_DEFAULT() {
    static lv_style_t *style;
    if (!style) {
        style = lv_mem_alloc(sizeof(lv_style_t));
        lv_style_init(style);
        init_style_hot_temp_MAIN_DEFAULT(style);
    }
    return style;
};

void add_style_hot_temp(lv_obj_t *obj) {
    (void)obj;
    lv_obj_add_style(obj, get_style_hot_temp_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

void remove_style_hot_temp(lv_obj_t *obj) {
    (void)obj;
    lv_obj_remove_style(obj, get_style_hot_temp_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

//
// Style: low_fuel
//

void init_style_low_fuel_MAIN_DEFAULT(lv_style_t *style) {
    lv_style_set_text_color(style, lv_color_hex(0xffff0000));
};

lv_style_t *get_style_low_fuel_MAIN_DEFAULT() {
    static lv_style_t *style;
    if (!style) {
        style = lv_mem_alloc(sizeof(lv_style_t));
        lv_style_init(style);
        init_style_low_fuel_MAIN_DEFAULT(style);
    }
    return style;
};

void add_style_low_fuel(lv_obj_t *obj) {
    (void)obj;
    lv_obj_add_style(obj, get_style_low_fuel_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

void remove_style_low_fuel(lv_obj_t *obj) {
    (void)obj;
    lv_obj_remove_style(obj, get_style_low_fuel_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

//
// Style: low_fuel_meter
//

void init_style_low_fuel_meter_MAIN_DEFAULT(lv_style_t *style) {
    lv_style_set_border_color(style, lv_color_hex(0xffff0000));
};

lv_style_t *get_style_low_fuel_meter_MAIN_DEFAULT() {
    static lv_style_t *style;
    if (!style) {
        style = lv_mem_alloc(sizeof(lv_style_t));
        lv_style_init(style);
        init_style_low_fuel_meter_MAIN_DEFAULT(style);
    }
    return style;
};

void add_style_low_fuel_meter(lv_obj_t *obj) {
    (void)obj;
    lv_obj_add_style(obj, get_style_low_fuel_meter_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

void remove_style_low_fuel_meter(lv_obj_t *obj) {
    (void)obj;
    lv_obj_remove_style(obj, get_style_low_fuel_meter_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

//
//
//

void add_style(lv_obj_t *obj, int32_t styleIndex) {
    typedef void (*AddStyleFunc)(lv_obj_t *obj);
    static const AddStyleFunc add_style_funcs[] = {
        add_style_tab_panel,
        add_style_tab_button,
        add_style_tab_active,
        add_style_hot_temp,
        add_style_low_fuel,
        add_style_low_fuel_meter,
    };
    add_style_funcs[styleIndex](obj);
}

void remove_style(lv_obj_t *obj, int32_t styleIndex) {
    typedef void (*RemoveStyleFunc)(lv_obj_t *obj);
    static const RemoveStyleFunc remove_style_funcs[] = {
        remove_style_tab_panel,
        remove_style_tab_button,
        remove_style_tab_active,
        remove_style_hot_temp,
        remove_style_low_fuel,
        remove_style_low_fuel_meter,
    };
    remove_style_funcs[styleIndex](obj);
}

