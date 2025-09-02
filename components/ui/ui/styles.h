#ifndef EEZ_LVGL_UI_STYLES_H
#define EEZ_LVGL_UI_STYLES_H

#include <lvgl.h>

#ifdef __cplusplus
extern "C" {
#endif

// Style: tab_panel
lv_style_t *get_style_tab_panel_MAIN_DEFAULT();
void add_style_tab_panel(lv_obj_t *obj);
void remove_style_tab_panel(lv_obj_t *obj);

// Style: tab_button
lv_style_t *get_style_tab_button_MAIN_DEFAULT();
void add_style_tab_button(lv_obj_t *obj);
void remove_style_tab_button(lv_obj_t *obj);

// Style: tab_active
lv_style_t *get_style_tab_active_MAIN_DEFAULT();
void add_style_tab_active(lv_obj_t *obj);
void remove_style_tab_active(lv_obj_t *obj);

// Style: hot_temp
lv_style_t *get_style_hot_temp_MAIN_DEFAULT();
void add_style_hot_temp(lv_obj_t *obj);
void remove_style_hot_temp(lv_obj_t *obj);

// Style: low_fuel
lv_style_t *get_style_low_fuel_MAIN_DEFAULT();
void add_style_low_fuel(lv_obj_t *obj);
void remove_style_low_fuel(lv_obj_t *obj);

// Style: low_fuel_meter
lv_style_t *get_style_low_fuel_meter_MAIN_DEFAULT();
void add_style_low_fuel_meter(lv_obj_t *obj);
void remove_style_low_fuel_meter(lv_obj_t *obj);



#ifdef __cplusplus
}
#endif

#endif /*EEZ_LVGL_UI_STYLES_H*/