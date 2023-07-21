import { useState } from 'react';
import { createStyles, Navbar, UnstyledButton, Tooltip, rem, Center, Stack, ThemeIcon, useMantineTheme, useMantineColorScheme } from '@mantine/core';
import {
  IconHome2,
  IconSwitchHorizontal,
  IconLogout,
  IconMathLower,
  IconMoon,
} from '@tabler/icons-react';
import { useRouter } from 'next/router';

const useStyles = createStyles((theme) => ({
  link: {
    width: rem(50),
    height: rem(50),
    borderRadius: theme.radius.md,
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    color: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.colors.gray[7],

    '&:hover': {
      backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[5] : theme.colors.gray[0],
    },
  },

  active: {
    '&, &:hover': {
      backgroundColor: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).background,
      color: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).color,
    },
  },
}));


const mockdata = [
  { icon: IconHome2, label: 'Dashboard', path: "/" },
];

interface NavbarLinkProps {
  icon: React.FC<any>;
  label: string;
  active?: boolean;
  onClick?(): void;
}

function NavbarLink({ icon: Icon, label, active, onClick }: NavbarLinkProps) {
  const { classes, cx } = useStyles();

  return (
    <Tooltip label={label} position="right" transitionProps={{ duration: 0 }}>
      <UnstyledButton onClick={onClick} className={cx(classes.link, { [classes.active]: active })}>
        <Icon size="1.2rem" stroke={1.5} />
      </UnstyledButton>
    </Tooltip>
  );
}

export function CustomNavbar() {
  const router = useRouter()
  const [active, setActive] = useState(0);
  const [darkMode, setDarkMode] = useState(false)
  // TODO: const { toggleColorScheme } = useMantineColorScheme();

  const handleDarkMode = () => {
    setDarkMode(!darkMode)
    // toggleColorScheme()
  }

  const links = mockdata.map((link, index) => (
    <NavbarLink
      {...link}
      key={link.label}
      active={index === active}
      onClick={() => router.push(link.path)}
    />
  ));

  return <Navbar width={{ base: 80 }} p="md">
    <Navbar.Section grow>
      <Stack justify="center">
        {links}
      </Stack>
    </Navbar.Section>
    <Navbar.Section>
      <Stack justify="center" spacing={0}>
        <NavbarLink icon={IconMoon} label="Dark mode" onClick={handleDarkMode} active={darkMode} />
        <NavbarLink icon={IconSwitchHorizontal} label="Change account" />
        <NavbarLink icon={IconLogout} label="Logout" />
      </Stack>
    </Navbar.Section>
  </Navbar>
}