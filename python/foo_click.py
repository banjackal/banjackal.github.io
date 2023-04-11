import click
from deprecated_option import DeprecatedOption, DeprecatedOptionsCommand

@click.command(cls=DeprecatedOptionsCommand)
@click.option('--some-flag', required=False, is_flag=True, cls=DeprecatedOption, deprecated=['--some-flag'], preferred='--new-flag')
@click.option('--name', prompt='Your name',
              help='The person to greet.')

def hello(some_flag, name):
	click.echo(f"Hello {name} - {some_flag}!")

if __name__ == '__main__':
    hello()


