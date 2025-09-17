from PIL import Image

img = Image.open('./mangabuddy/res/icon.webp').convert('RGB')
img = img.resize((128, 128))
img.save('./mangabuddy/res/icon.png', 'PNG')
