import { Container, Flex, Space, useMantineTheme } from "@mantine/core"
import { CustomNavBar } from "./CustomNavBar"

interface CustomPageProps {
    title: any
    children: any
}

export default function CustomPage({ title, children }: CustomPageProps) {
    const theme = useMantineTheme()
    return (
        <Flex bg={theme.colors.gray[1]} >
            <CustomNavBar />
            <Container mt="5rem" size="lg">
                {title}
                <Space h="xl" />
                {children}
            </Container>
        </Flex>
    )
}