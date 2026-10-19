#if TMNEXT
namespace Example {
    enum Mode { First, Second }
    class Counter {
        uint value = 0;
        void Add(const string &in label, uint &out result) {
            value++;
            result = value;
            print(label);
        }
    }
    void Visit(Counter@ counter, Counter@ &in handle, int &inout count) {
        array<string> labels = {"one", "two"};
        count += 1;
        bool present = counter !is null;
    }
}
#endif
