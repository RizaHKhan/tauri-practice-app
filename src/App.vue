<template>
    <div class="app">
        <Tabs :tabs="tabs">
            <template #content-style>
                <Style />
            </template>

            <template #content-other>
                <div class="cards">
                    <Card title="Title">
                        <p>
                            Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint
                            ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur
                            officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate
                            dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea
                            nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat
                            officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis
                            officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim.
                            Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.
                        </p>
                        <template #footer>
                            <Button>Do This</Button>
                            <Button secondary>No</Button>
                        </template>
                    </Card>
                    <Card>
                        <template #header>
                            <h1>Custom Header</h1>
                        </template>
                        <p>
                            Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint
                            ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur
                            officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate
                            dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea
                            nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat
                            officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis
                            officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim.
                            Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.
                        </p>
                    </Card>
                    <Card>
                        <template #header>
                            <h1>Custom Header</h1>
                        </template>
                        <p>
                            Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint
                            ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur
                            officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate
                            dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea
                            nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat
                            officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis
                            officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim.
                            Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.
                        </p>
                    </Card>
                </div>
            </template>
        </Tabs>
    </div>
</template>

<script setup lang="ts">
import { Tab } from 'types'
import Style from 'views/Style.vue'
import Tabs from 'atoms/Tabs.vue'
import Card from 'atoms/Card.vue'
import Button from 'atoms/Button.vue'

const tabs: Tab[] = [
    {
        name: 'style',
        title: 'Style',
    },
    {
        name: 'other',
        title: 'Other',
    },
]
</script>

<style lang="scss" scoped>
.app {
    @include padding(md);
}

.cards {
    @include grid-container(1fr 1fr);
}
</style>
