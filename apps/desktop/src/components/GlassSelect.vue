<script setup lang="ts">
import {
  computed,
  nextTick,
  onBeforeUnmount,
  onMounted,
  ref,
  useId,
} from 'vue';

import {
  edgeEnabledOptionIndex,
  moveEnabledOptionIndex,
  selectedEnabledOptionIndex,
  type GlassSelectOption,
} from '../domain/glass-controls.ts';

const props = withDefaults(
  defineProps<{
    modelValue: string;
    options: readonly GlassSelectOption[];
    label: string;
    disabled?: boolean;
    emptyLabel?: string;
  }>(),
  {
    disabled: false,
    emptyLabel: 'Select an option',
  },
);

const emit = defineEmits<{
  'update:modelValue': [value: string];
  change: [value: string];
}>();

const root = ref<HTMLElement | null>(null);
const trigger = ref<HTMLButtonElement | null>(null);
const open = ref(false);
const activeIndex = ref(-1);

const componentId = useId();
const listboxId = `glass-select-listbox-${componentId}`;

const selectedOption = computed(() =>
  props.options.find((option) => option.value === props.modelValue),
);

const activeOptionId = computed(() =>
  open.value && activeIndex.value >= 0
    ? `${listboxId}-option-${activeIndex.value}`
    : undefined,
);

function syncActiveIndex(): void {
  activeIndex.value = selectedEnabledOptionIndex(
    props.options,
    props.modelValue,
  );
}

function openList(): void {
  const hasEnabledOption = props.options.some(
    (option) => !option.disabled,
  );

  if (props.disabled || !hasEnabledOption) return;

  syncActiveIndex();
  open.value = true;
}

function closeList(
  { restoreFocus = false }: { restoreFocus?: boolean } = {},
): void {
  open.value = false;
  activeIndex.value = -1;

  if (restoreFocus) {
    void nextTick(() => trigger.value?.focus());
  }
}

function toggleList(): void {
  if (open.value) closeList();
  else openList();
}

function activateIndex(index: number): void {
  if (!props.options[index]?.disabled) {
    activeIndex.value = index;
  }
}

function selectIndex(index: number): void {
  const option = props.options[index];

  if (!option || option.disabled) return;

  emit('update:modelValue', option.value);
  emit('change', option.value);
  closeList({ restoreFocus: true });
}

function move(direction: -1 | 1): void {
  if (!open.value) {
    openList();
  }

  activeIndex.value = moveEnabledOptionIndex(
    props.options,
    activeIndex.value,
    direction,
  );
}

function handleKeydown(event: KeyboardEvent): void {
  switch (event.key) {
    case 'ArrowDown':
      event.preventDefault();
      move(1);
      break;

    case 'ArrowUp':
      event.preventDefault();
      move(-1);
      break;

    case 'Home':
      event.preventDefault();

      if (!open.value) openList();

      activeIndex.value = edgeEnabledOptionIndex(
        props.options,
        'first',
      );
      break;

    case 'End':
      event.preventDefault();

      if (!open.value) openList();

      activeIndex.value = edgeEnabledOptionIndex(
        props.options,
        'last',
      );
      break;

    case 'Enter':
    case ' ':
      event.preventDefault();

      if (!open.value) {
        openList();
      } else {
        selectIndex(activeIndex.value);
      }
      break;

    case 'Escape':
      if (open.value) {
        event.preventDefault();
        closeList({ restoreFocus: true });
      }
      break;

    case 'Tab':
      closeList();
      break;
  }
}

function handleDocumentPointerDown(event: PointerEvent): void {
  if (!root.value?.contains(event.target as Node | null)) {
    closeList();
  }
}

onMounted(() => {
  document.addEventListener(
    'pointerdown',
    handleDocumentPointerDown,
  );
});

onBeforeUnmount(() => {
  document.removeEventListener(
    'pointerdown',
    handleDocumentPointerDown,
  );
});
</script>

<template>
  <div
    ref="root"
    class="glass-select"
    :class="{
      open,
      disabled: props.disabled,
    }"
  >
    <button
      ref="trigger"
      class="glass-select__trigger"
      type="button"
      role="combobox"
      aria-haspopup="listbox"
      :aria-label="props.label"
      :aria-expanded="open"
      :aria-controls="listboxId"
      :aria-activedescendant="activeOptionId"
      :disabled="props.disabled"
      @click="toggleList"
      @keydown="handleKeydown"
    >
      <span class="glass-select__value">
        {{ selectedOption?.label ?? props.emptyLabel }}
      </span>

      <span
        class="glass-select__chevron"
        aria-hidden="true"
      >
        ⌄
      </span>
    </button>

    <div
      v-if="open"
      :id="listboxId"
      class="glass-select__listbox"
      role="listbox"
      :aria-label="props.label"
    >
      <button
        v-for="(option, index) in props.options"
        :id="`${listboxId}-option-${index}`"
        :key="option.value"
        class="glass-select__option"
        :class="{
          active: activeIndex === index,
          selected: option.value === props.modelValue,
        }"
        type="button"
        role="option"
        :aria-selected="option.value === props.modelValue"
        :disabled="option.disabled"
        @mouseenter="activateIndex(index)"
        @click="selectIndex(index)"
      >
        <span class="glass-select__option-copy">
          <strong>{{ option.label }}</strong>

          <small v-if="option.description">
            {{ option.description }}
          </small>
        </span>

        <span
          v-if="option.value === props.modelValue"
          class="glass-select__check"
          aria-hidden="true"
        >
          <svg
            class="glass-select__check-icon"
            viewBox="0 0 16 16"
            fill="none"
            focusable="false"
          >
            <path
              d="M3.25 8.25 6.45 11.2 12.75 4.75"
              stroke="currentColor"
              stroke-width="1.8"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
        </span>
      </button>
    </div>
  </div>
</template>
