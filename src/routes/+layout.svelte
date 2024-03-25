<script lang="ts">
	import { Container, Navbar, NavbarBrand, Button, colorMode, Icon, ThemeToggler, Styles, Collapse, Dropdown, DropdownItem, DropdownMenu, DropdownToggle, Nav, NavItem, NavLink, NavbarToggler } from '@sveltestrap/sveltestrap';

	
	function isDarkMode(): string {
		if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    		return 'dark';
 } else {
    return 'light';
 }
	}

	// import { TrayIcon } from '@tauri-apps/api/tray';

	// let tray: TrayIcon;

	// async function showTrayIcon() {
	// 	tray = await TrayIcon.new({ tooltip: 'awesome tray tooltip' });
	// 	tray.setTooltip('new tooltip');
	// }

	// showTrayIcon();

	// <header role="heading" aria-level="1" on:mouseenter={() => console.log("Mouse entered the header console log")}>
	let isOpen = false;

	function handleUpdate(event: any) {
  		isOpen = event.detail.isOpen;
	}


	const theme = 'dark'; // Should get from media query / user settings / toggle button and save user setting
</script>

<Styles />

<header class="gradient-custom">
	<!-- add color="primary-subtle" to Navbar if using normal color scheme -->
<Navbar {theme} expand="md" container="xl" >
	<NavbarBrand href="/">Brand Name/Logo</NavbarBrand>
	<NavbarToggler on:click={() => (isOpen = !isOpen)} />
		<Collapse {isOpen} navbar expand="md" on:update={handleUpdate}>
		  <Nav underline class="ms-auto" navbar>
			<NavItem>
			  <NavLink href="#components/">Components</NavLink>
			</NavItem>
			<NavItem>
			  <NavLink href="https://github.com/sveltestrap/sveltestrap">GitHub</NavLink>
			</NavItem>
			<Dropdown nav inNavbar>
			  <DropdownToggle nav caret>Settings</DropdownToggle>
			  <DropdownMenu end>
				<DropdownItem>Setting 1</DropdownItem>
				<DropdownItem>Setting 2</DropdownItem>
				<DropdownItem divider />
				<DropdownItem>Reset</DropdownItem>
			  </DropdownMenu>
			</Dropdown>
		  </Nav>
		</Collapse>
</Navbar>
</header>





I should update the sveltestrap package since last week it was updated?




<!-- <header role="heading" aria-level="1" on:mouseenter={(e) => console.log("Mouse entered the header console log", e.type, e.pageX, "Window query:", isDarkMode())}>
	
	<Navbar dark expand="md" >
		<Button outline color="primary">Button1</Button>
		<Button color="secondary">Button2</Button>
		<Button outline color="success">Button3</Button>
		<Button outline color="info">Button4</Button>
		<Button outline color="danger">Button5</Button>
		<Button color="warning">Button6</Button>
	</Navbar>
</header> -->

<div class="icon-example">
	<Icon name="layout-text-window-reverse"/>
</div>



<slot />

<h1>
Create LAPCE workspace, and why are my html tags not auto closing?
</h1>

<footer >
	<h1>Testing</h1>

	<Button
		color="primary"
		outline
		active={$colorMode === 'light'}
		on:click={() => ($colorMode = 'light')}
	>
		light <Icon name="sun-fill" />
	</Button>
	<Button
		color="primary"
		outline
		active={$colorMode === 'dark'}
		on:click={() => ($colorMode = 'dark')}
	>
		dark <Icon name="moon-stars-fill" />
	</Button>
	<Button
		color="primary"
		outline
		active={$colorMode === 'auto'}
		on:click={() => ($colorMode = 'auto')}
	>
		auto <Icon name="circle-half" />
	</Button>

	// @ts-expect-error: No declaration file for module
	<ThemeToggler let:currentColorMode let:toggleColorMode>
		<Button color="primary" on:click={() => toggleColorMode()}>
			Toggle Theme: {currentColorMode}
		</Button>
	</ThemeToggler>
</footer>

<style>
	/* .nav-container {
		padding: 1em;
	} */

	footer {
		padding: 4em;
	}

	.icon-example {
		font-size: 5vw; /* This is relative to the size of the view / window */
		color: aqua;
	}
	.gradient-custom {
  /* fallback for old browsers */
  background: #6a11cb;

  /* Chrome 10-25, Safari 5.1-6 */
  background: -webkit-linear-gradient(to right, rgba(106, 17, 203, 0.9), rgba(37, 117, 252, 0.9));

  /* W3C, IE 10+/ Edge, Firefox 16+, Chrome 26+, Opera 12+, Safari 7+ */
  background: linear-gradient(to right, rgba(106, 17, 203, 0.9), rgba(37, 117, 252, 0.9));
}

</style>
