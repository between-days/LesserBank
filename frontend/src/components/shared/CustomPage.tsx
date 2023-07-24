import { Center, Container, Flex, Group, Space, Stack, useMantineTheme } from "@mantine/core"
import { CustomNavBar } from "./CustomNavBar"

interface CustomPageProps {
    title: any
    children: any
}

export default function CustomPage({ title, children }: CustomPageProps) {
    const theme = useMantineTheme()
    return (
        <Flex bg={theme.colors.gray[1]}>
            <CustomNavBar />
            <Container mt="6rem">
                <Stack>
                    {title}
                    {children}
                </Stack>
            </Container>
        </Flex>
    )
}