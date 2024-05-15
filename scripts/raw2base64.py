import sys

import rawpy
from PIL import Image
import io
import base64

# 打开 NEF 图像文件
with rawpy.imread(sys.argv[1]) as raw:
    # 转换为 RGB 图像
    rgb = raw.postprocess()
    # 创建 Pillow 图像对象
    pil_image = Image.fromarray(rgb)

    # 控制分辨率为 800x600，保持原始比例
    pil_image.thumbnail((640, -1))

    # 将 PIL 图像对象转换为字节流
    with io.BytesIO() as buffer:
        pil_image.save(buffer, format="JPEG")
        image_bytes = buffer.getvalue()

    # 保存调整尺寸后的图像
    base64_image = base64.b64encode(image_bytes).decode('utf-8')

    # 打印 base64 编码字符串
    print(base64_image)

