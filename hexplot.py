import json
import math
import sys

from PIL import Image, ImageDraw, ImageFont


class HexagonGenerator(object):
    def __init__(self, width, height):
        self.width = width
        self.height = height

    def __call__(self, row, col, edge):
        x = 20 + (col + .5 * (row % 2)) * math.sqrt(3) * edge
        y = row * 2 * (edge - 4)
        for angle in range(30, 360, 60):
            x += math.cos(math.radians(angle)) * edge
            y += math.sin(math.radians(angle)) * edge
            yield x
            yield y


def main(path, edge=16):
    try:
        font = ImageFont.truetype("arial.ttf", 7)
    except:
        font = ImageFont.truetype("Arial.ttf", 7)
    for i, data in enumerate(json.load(open(path))):
        board, unit = data["board"], data["unit"]
        hexagon_generator = HexagonGenerator(board["width"], board["height"])
        cells = board["cells"]
        image = Image.new('RGBA', (board["width"] * edge * 2,
                                   board["height"] * edge * 2),
                          'white')
        draw = ImageDraw.Draw(image)
        for row in range(board["height"]):
            for col in range(board["width"]):
                if any(x == col and y == row for x, y in unit["cells"]):
                    color, outline = "black", "white"
                elif cells[row][col]:
                    color, outline = "yellow", "black"
                else:
                    color, outline = "white", "black"

                if unit["pivot"] == [col, row]:
                    outline = "red"

                coords = list(hexagon_generator(row, col, edge))
                draw.polygon(coords, outline=outline, fill=color)
                mx = sum(coords[::2]) * 2 / len(coords) - edge / 2
                my = sum(coords[1::2]) * 2 / len(coords) - edge / 4
                draw.text((mx, my), str((col, row)), fill=outline, font=font)

        draw.text((image.width // 2, image.height - 20),
                  data["previous_move"], font=font, fill="black")
        image.save("./solution/step_{:08d}.png".format(i))


if __name__ == "__main__":
    main(*sys.argv[1:])
