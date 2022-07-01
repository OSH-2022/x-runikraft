from PIL import Image
import numpy as np

img = Image.open('cursor16x16.png')
if img.mode != 'RGBA':
    img = img.convert("RGBA")
img = np.array(img)  # 打开图像并转化为数字矩阵
with open("cursor16x16.txt", "w") as file:
    for i in range(img.shape[0]):
        for j in range(img.shape[1]):
            file.write(str(img[i][j]) + '\n')
# print(img.shape)
