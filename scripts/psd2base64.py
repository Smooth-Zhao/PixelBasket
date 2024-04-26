import sys
from psd_tools import PSDImage
import io
import base64

psd = PSDImage.open(sys.argv[1])

image = psd.composite()
image.save('exported.jpg')

