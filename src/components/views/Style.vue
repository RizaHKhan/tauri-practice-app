<script setup lang="ts">
import { ref } from 'vue'
import Button from 'atoms/Button.vue'
import Input from 'atoms/Input.vue'

const input = ref('')
</script>

<template>
    <div class="container">
        <div class="typography">
            <strong>Typography:</strong>
            <div class="content">
                <h1>h1</h1>
                <h2>h2</h2>
                <h3>h3</h3>
                <h4>h4</h4>
                <h5>h5</h5>
                <p>p</p>
            </div>

            <div class="content">
                <p class="goobar">
                    Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi
                    Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia.
                    Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est
                    proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat
                    reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident
                    adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit
                    commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea
                    consectetur et est culpa et culpa duis.
                </p>
                <p class="foobar">
                    Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi
                    Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia.
                    Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est
                    proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat
                    reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident
                    adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit
                    commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea
                    consectetur et est culpa et culpa duis.
                </p>
            </div>
        </div>

        <div class="buttons">
            <strong>Buttons:</strong>
            <div class="content">
                <Button>Primary</Button>
                <Button secondary>Secondary</Button>
                <Button warn>Primary</Button>
            </div>

            <div class="content">
                <Button disabled>Primary</Button>
                <Button disabled secondary>Secondary</Button>
                <Button disabled warn>Primary</Button>
            </div>

            <div class="content">
                <Button secondary sm>Secondary</Button>
                <Button warn sm>Warn</Button>
                <Button warn sm>Warn</Button>
            </div>

            <div class="content">
                <Button secondary lg>Secondary</Button>
                <Button warn lg>Warn</Button>
                <Button warn lg>Warn</Button>
            </div>
        </div>
        <div class="inputs">
            {{ input }}
            <div class="form">
                <Input v-model="input" label="Name" placeholder="foo" />
                <Input v-model="input" email label="Email" placeholder="foo" />
                <Input v-model="input" label="Password" password placeholder="Password" />
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.content {
    @include flex-container(row);
    @include margin(md, bottom);
}

.form {
    @include grid-container(1fr 1fr, auto, sm);
}
</style>
