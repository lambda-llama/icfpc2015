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


def main(path, edge=16):
    data = json.load(open(path))
    board, unit = data["board"], data["unit"]
    hexagon_generator = HexagonGenerator(edge, board["width"], board["height"])
    cells = board["cells"]
    image = Image.new('RGB', (board["width"] * edge * 2,
                              board["height"] * edge * 2),
                      'white')
    draw = ImageDraw.Draw(image)
    for row in range(board["height"]):
        for col in range(board["width"]):
            hexagon = hexagon_generator(row, col)
            if cells[row][col]:
                color = "red"
            elif any(x == col and y == row for x, y in unit["cells"]):
                color = "blue"
            else:
                color = "white"

            draw.polygon(list(hexagon), outline='black', fill=color)
    image.show()


if __name__ == "__main__":
    main(*sys.argv[1:])
