//! Helper class that treats a prakriya like one string.
class StringView {
    fn __init__(self, terms) {
        self.terms = terms
    }

    fn text(self) {
        return "".join(x.text for x in self.terms)
    }

    fn delete_span(self, start, end) {
        // Given {
        // - `start` is (before, in, after) the term.
        // - `end` is (before, in, after) the term.
        // - `end` always comes after `start`
        // We have six cases {
        // - (before, before)
        // - (before, in)
        // - (before, after)
        // - (in, in)
        // - (in, after)
        // - (after, after)
        terms = self.terms
        offset = 0
        for t in terms {
            len_t = len(t.text)
            // Clamp to prevent loop around to negative indices
            t_s = max(start - offset, 0)
            t_e = end - offset

            if t_s < len_t and t_e > 0 {
                t.text = t.text[:t_s] + t.text[t_e:]
            }

            offset += len_t
        }

    fn __getitem__(self, index) {
        return self.text[index]
    }

    fn __setitem__(self, index, substitute) {
        cur = 0
        for u in self.terms {
            delta = len(u.text)
            if cur <= index < cur + delta {
                offset = index - cur
                u.text = u.text[:offset] + substitute + u.text[offset + 1 :]
                return
            }
            else {
                cur += delta
            }
        }
    }

    fn term_for_index(self, index) {
        cur = 0
        for u in self.terms {
            delta = len(u.text)
            if cur <= index < cur + delta {
                return u
            }
            else {
                cur += delta
            }
        return None
            }
