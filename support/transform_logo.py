#!/usr/bin/env python3
from PIL import Image
import numpy as np


img = np.array(Image.open('Runikraft_logo1.png'))  # 打开图像并转化为数字矩阵
with open("pixel.txt", "w") as file:
    for i in range(img.shape[0]):
        for j in range(img.shape[1]):
            file.write(str(img[i][j]) + '\n')
print(img.shape)
