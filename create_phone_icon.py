#!/usr/bin/env python3
"""
Create a cell phone icon for the eSubpoena Tolls Tool executable.
This script generates a simple cell phone icon in ICO format.
"""

from PIL import Image, ImageDraw, ImageFont
import os

def create_phone_icon():
    """Create a cell phone icon and save it as ICO file."""
    
    # Create a 256x256 image with transparent background
    size = 256
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Phone body (rounded rectangle)
    phone_width = int(size * 0.6)
    phone_height = int(size * 0.8)
    phone_x = (size - phone_width) // 2
    phone_y = (size - phone_height) // 2
    
    # Draw phone body with rounded corners
    corner_radius = 20
    
    # Main body
    draw.rounded_rectangle(
        [phone_x, phone_y, phone_x + phone_width, phone_y + phone_height],
        radius=corner_radius,
        fill=(50, 50, 50),
        outline=(30, 30, 30),
        width=3
    )
    
    # Screen area
    screen_margin = 15
    screen_x = phone_x + screen_margin
    screen_y = phone_y + screen_margin
    screen_width = phone_width - 2 * screen_margin
    screen_height = phone_height - 2 * screen_margin - 40  # Leave space for buttons
    
    draw.rounded_rectangle(
        [screen_x, screen_y, screen_x + screen_width, screen_y + screen_height],
        radius=corner_radius // 2,
        fill=(100, 150, 200),
        outline=(80, 120, 160),
        width=2
    )
    
    # Home button (circle at bottom)
    button_center_x = phone_x + phone_width // 2
    button_center_y = phone_y + phone_height - 25
    button_radius = 12
    
    draw.ellipse(
        [button_center_x - button_radius, button_center_y - button_radius,
         button_center_x + button_radius, button_center_y + button_radius],
        fill=(80, 80, 80),
        outline=(60, 60, 60),
        width=2
    )
    
    # Add some screen content (simple grid pattern)
    grid_spacing = 20
    for x in range(screen_x + 10, screen_x + screen_width - 10, grid_spacing):
        for y in range(screen_y + 10, screen_y + screen_height - 10, grid_spacing):
            draw.rectangle([x, y, x + 15, y + 15], fill=(120, 170, 220))
    
    # Add a small antenna or signal indicator
    signal_x = phone_x + phone_width + 5
    signal_y = phone_y + 20
    for i in range(3):
        bar_height = 8 + i * 4
        draw.rectangle(
            [signal_x + i * 3, signal_y + 20 - bar_height, 
             signal_x + i * 3 + 2, signal_y + 20],
            fill=(0, 200, 0)
        )
    
    # Save as ICO with multiple sizes
    icon_sizes = [16, 32, 48, 64, 128, 256]
    icon_images = []
    
    for icon_size in icon_sizes:
        resized_img = img.resize((icon_size, icon_size), Image.Resampling.LANCZOS)
        icon_images.append(resized_img)
    
    # Save as ICO file
    icon_path = "phone_icon.ico"
    icon_images[0].save(
        icon_path,
        format='ICO',
        sizes=[(size, size) for size in icon_sizes],
        append_images=icon_images[1:]
    )
    
    print(f"✅ Cell phone icon created: {icon_path}")
    return icon_path

if __name__ == "__main__":
    create_phone_icon() 