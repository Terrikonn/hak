#!/usr/bin/env python

from argparse import *

ARCHITECTURES = ["x86_64", "riscv"]


class CapitalisedHelpFormatter(HelpFormatter):
    def add_usage(self, usage, actions, groups, prefix=None):
        if prefix is None:
            prefix = "Usage: "
        return super(CapitalisedHelpFormatter, self).add_usage(
            usage, actions, groups, prefix
        )


class Subcommand:
    def __init__(self, ap: ArgumentParser):
        self.parser = ap
        # Format case of help message
        self.parser._optionals.title = "Optional arguments"
        self.parser._positionals.title = "Positional arguments"
        self.parser.add_argument(
            "-h",
            "--help",
            action="help",
            default=SUPPRESS,
            help="Show this help message and exit.",
        )

    def execute(self):
        pass


class Build(Subcommand):
    def __init__(self, ap: ArgumentParser):
        super().__init__(ap)
        self.parser.add_argument(
            "--target", choices=ARCHITECTURES, help="choose architecture"
        )

    def execute(self):
        pass


class Arguments:
    def __init__(self) -> None:
        self.parser: ArgumentParser = ArgumentParser(
            description="Build kernel",
            formatter_class=CapitalisedHelpFormatter,
            add_help=False,
        )

        # Format case of help message
        self.parser._optionals.title = "Optional arguments"
        self.parser._positionals.title = "Positional arguments"
        self.parser.add_argument(
            "-v",
            "--version",
            action="version",
            version="%(prog)s 1.0",
            help="Show program's version number and exit.",
        )
        self.parser.add_argument(
            "-h",
            "--help",
            action="help",
            default=SUPPRESS,
            help="Show this help message and exit.",
        )

        # Add subcommands
        self.subparsers = self.parser.add_subparsers(title="Subcommands")
        self.build_subcommand = Build(
            self.subparsers.add_parser("build", help="help build")
        )
        self.run_subcommand = self.subparsers.add_parser("run", help="run kernel")
        self.run_subcommand.add_argument(
            "--target", choices=ARCHITECTURES, help="choose archtecture"
        )
        self.check_subcommand = self.subparsers.add_parser("check", help="help check")
        self.check_subcommand.add_argument(
            "--target", choices=ARCHITECTURES, help="choose archtecture"
        )

    def parse(self) -> Namespace:
        return self.parser.parse_args()


def main() -> None:
    args = Arguments().parse()

    print(args.target)

    return None


if __name__ == "__main__":
    main()
