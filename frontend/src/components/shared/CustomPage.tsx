import { Center, Container, Flex, useMantineTheme } from "@mantine/core"
import { CustomNavBarAlt2 } from "./CustomNavBarAlt2"

interface CustomPageProps {
    title: string
    children: any
}

export default function CustomPage({ title, children }: CustomPageProps) {
    const theme = useMantineTheme()
    return (
        <Flex bg={theme.colors.gray[1]} >
            <CustomNavBarAlt2 />
            <Container mt="5rem" size="lg">
                {children}
            </Container>
        </Flex>
    )
}