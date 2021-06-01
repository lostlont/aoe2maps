#!/usr/bin/env python

import glob
from gimpfu import *

class Entry:
	def __init__(self):
		self.out_filename = ''
		self.down_filename = ''
		self.up_filename = ''

class ImageGuard:
	def __init__(self, image):
		self.image = image
	
	def __enter__(self):
		return self

	def __exit__(self, exception_type, exception_value, traceback):
		pdb.gimp_image_delete(self.image)

def screenshot_to_icon(file_pattern):
	for entry in list_images(file_pattern):
		image, down_layer, up_layer = load_images(entry.down_filename, entry.up_filename)
		with ImageGuard(image):
			layer = cut_upper_half(image, up_layer)
			map_selection = crop_map(image, layer)
			layer = add_gradient(image, map_selection)
			layer = add_border(image, map_selection)
			layer = add_shadow(image, layer)
			save_image(image, layer, entry.out_filename)

def load_images(down_filename, up_filename):
	image = pdb.gimp_file_load(down_filename, down_filename)

	down_layer = pdb.gimp_image_get_active_layer(image)
	
	up_layer = pdb.gimp_file_load_layer(image, up_filename)
	pdb.gimp_image_insert_layer(image, up_layer, None, -1)

	return image, down_layer, up_layer

def save_image(image, layer, out_filename):
	pdb.gimp_file_save(image, layer, out_filename, out_filename)

def cut_upper_half(image, layer):
	pdb.gimp_image_select_rectangle(image, CHANNEL_OP_REPLACE, 1526, 896, 394, 90)
	pdb.gimp_drawable_edit_clear(layer)
	return merge_image(image)

def merge_image(image):
	return pdb.gimp_image_merge_visible_layers(image, CLIP_TO_IMAGE)

def crop_map(image, layer):
	points = [
		1557, 985,
		1729, 892,
		1902, 985,
		1729, 1078,
	]
	pdb.gimp_image_select_polygon(image, CHANNEL_OP_REPLACE, len(points), points)
	map_selection = pdb.gimp_selection_save(image)
	pdb.gimp_selection_invert(image)
	pdb.gimp_drawable_edit_clear(layer)
	pdb.gimp_selection_none(image)
	pdb.plug_in_autocrop(image, layer)
	return map_selection

def add_gradient(image, map_selection):
	width = pdb.gimp_image_width(image)
	height = pdb.gimp_image_height(image)
	gradient_layer = pdb.gimp_layer_new(image, width, height, RGBA_IMAGE, 'gradient', 100, LAYER_MODE_HARDLIGHT)
	pdb.gimp_image_insert_layer(image, gradient_layer, None, -1)
	pdb.gimp_drawable_edit_gradient_fill(gradient_layer, GRADIENT_LINEAR, 0, False, 0, 0, False, width / 2, height / 2 + 3, width / 2, height / 2 - 3)
	pdb.gimp_drawable_brightness_contrast(gradient_layer, 0.0, -0.25)
	pdb.gimp_image_select_item(image, CHANNEL_OP_REPLACE, map_selection)
	pdb.gimp_selection_shrink(image, 3)
	pdb.gimp_drawable_edit_clear(gradient_layer)
	pdb.gimp_selection_none(image)
	return merge_image(image)

def add_border(image, map_selection):
	pdb.gimp_image_resize(image, 445, 286, 50, 50)
	for layer in image.layers:
		pdb.gimp_layer_resize_to_image_size(layer)

	border_layer = pdb.gimp_layer_new(image, image.width, image.height, RGBA_IMAGE, 'border', 66, LAYER_MODE_NORMAL)
	pdb.gimp_image_insert_layer(image, border_layer, None, 1)
	
	pdb.gimp_image_select_item(image, CHANNEL_OP_REPLACE, map_selection)
	pdb.gimp_selection_grow(image, 2)
	pdb.gimp_context_push()
	pdb.gimp_context_set_foreground((0, 0, 0))
	pdb.gimp_drawable_edit_fill(border_layer, FILL_FOREGROUND)
	pdb.gimp_context_pop()
	pdb.gimp_selection_none(image)

	pdb.plug_in_autocrop(image, border_layer)
	return merge_image(image)

def add_shadow(image, layer):
	pdb.script_fu_drop_shadow(image, layer, 0.0, 5.0, 10.0, (0, 0, 0), 50, True)
	return merge_image(image)

def list_images(file_pattern):
	filenames = glob.glob(file_pattern)
	for down_filename in filenames:
		down_suffix = '-down.png'
		if down_filename.endswith(down_suffix):
			filename = down_filename[:-len(down_suffix)]
			up_filename = '{}-up.png'.format(filename)
			if filenames.count(up_filename) != 0:
				entry = Entry()
				entry.out_filename = 'output\\{}.png'.format(filename)
				entry.down_filename = down_filename
				entry.up_filename = up_filename
				yield entry

register("python-screenshot-to-icon", "", "", "", "", "", "", "", [
	(PF_STRING, "file_pattern", "file_pattern", "")
], [], screenshot_to_icon)

main()
