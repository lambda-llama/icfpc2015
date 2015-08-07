import json
import math
import sys

from PIL import Image, ImageDraw


class HexagonGenerator(object):
    def __init__(self, edge, width, height):
        self.edge = edge
        self.width = width
        self.height = height

    def __call__(self, row, col):
        x = (col + 1 - .5 * (row % 2)) * math.sqrt(3) * self.edge
        y = row * 2 * self.edge
        for angle in range(30, 360, 60):
            x += math.cos(math.radians(angle)) * self.edge
            y += math.sin(math.radians(angle)) * self.edge
            yield x
            yield y


def main(path):
    board = json.load(open(path))
    image = Image.new('RGB', (256, 256), 'white')
    draw = ImageDraw.Draw(image)
    hexagon_generator = HexagonGenerator(16, board["width"], board["height"])
    cells = board["cells"]
    for row in range(board["height"]):
        for col in range(board["width"]):
            hexagon = hexagon_generator(row, col)
            draw.polygon(list(hexagon), outline='black',
                         fill='red' if cells[row][col] else 'white')
    image.show()


if __name__ == "__main__" :
    main(*sys.argv[1:])
