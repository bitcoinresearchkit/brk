(async () => {
  const theme = await (
    await fetch(
      "https://raw.githubusercontent.com/tailwindlabs/tailwindcss/refs/heads/next/packages/tailwindcss/theme.css",
    )
  ).text();

  console.log(
    [
      "red",
      "orange",
      "amber",
      "yellow",
      "lime",
      "green",
      "emerald",
      "teal",
      "cyan",
      "sky",
      "blue",
      "indigo",
      "violet",
      "purple",
      "fuchsia",
      "pink",
      "rose",
    ]
      .map((color) => {
        const [a, b] = [500, 600].map((shade) => {
          const regExp = new RegExp(
            `(?<=${`${color}-${shade}: oklch\(`})(.*?)(?=\\s*${`\);`})`,
            "g",
          );
          let res = regExp.exec(theme)?.[2];
          if (!res) throw "err";
          res = res.replace("(", "");
          res = res.replace(")", "");
          // return res
          return res.split(" ").map((s) => Number(s));
        });
        const mult = 10_000;
        return `--${color}: oklch(${[0, 1, 2].map((i) => Math.round(((a[i] + b[i]) / 2) * mult) / mult).join(" ")})`;
      })
      .join(";\n"),
  );
})();
