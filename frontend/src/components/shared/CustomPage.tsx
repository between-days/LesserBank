import { AppShell, Center, MediaQuery, Stack, useMantineTheme } from "@mantine/core"
import { CustomNavBar } from "./CustomNavBar"

interface CustomPageProps {
    title: any
    children: any
}

export default function CustomPage({ title, children }: CustomPageProps) {
    const theme = useMantineTheme()
    return (
        <AppShell
            p={0}
            bg={theme.colors.gray[1]}
            navbar={
                <MediaQuery smallerThan="lg" styles={{ display: 'none' }} >
                    <CustomNavBar />
                </MediaQuery>
            }>
            <Center mt="6rem">
                <Stack w="90%">
                    {title}
                    {children}
                </Stack>
            </Center>
        </AppShell>
    )
}