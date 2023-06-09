use criterion::{criterion_group, criterion_main, Criterion};
use solver::longest_substring_without_repeating_characters::length_of_longest_substring as func;
use solver::regular_expression_matching::is_match as func2;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lswrc", |b| b.iter(|| func("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque massa orci, imperdiet sit amet venenatis ac, commodo ut lacus. Nullam nulla est, pharetra quis massa ut, imperdiet egestas neque. Nullam scelerisque, turpis nec sollicitudin venenatis, nulla elit pharetra lectus, sit amet faucibus justo enim at mauris. Integer vitae cursus nibh. Etiam ac erat sem. Nullam ut facilisis turpis. Aliquam semper lacus leo, quis ultrices enim auctor sit amet.

    Nam nec risus gravida, fermentum nulla eu, ultricies diam. Morbi ut lacinia leo. Phasellus condimentum eros non ante accumsan, in aliquet nibh viverra. Nullam ullamcorper ligula sed mauris viverra tincidunt. Mauris odio dui, volutpat ut elementum at, consectetur nec ex. Etiam sit amet tellus congue, ornare orci vitae, sollicitudin mauris. Vestibulum velit risus, dapibus id bibendum et, tincidunt eu nulla. Aenean maximus nec eros a pellentesque. Sed feugiat ornare metus quis faucibus. Praesent tristique neque suscipit consequat egestas. Donec eget laoreet tellus.
    
    Sed ullamcorper metus porttitor ex interdum, vitae laoreet lacus venenatis. Interdum et malesuada fames ac ante ipsum primis in faucibus. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Pellentesque varius nulla vel libero tempus, non dapibus dui imperdiet. Proin ut molestie dui. Curabitur sit amet urna vitae elit faucibus aliquam quis ut turpis. Nunc ac tincidunt urna. Nulla rhoncus tellus id dolor imperdiet consectetur. Vivamus ornare malesuada feugiat. Integer rutrum sit amet tellus sit amet pretium. Praesent egestas posuere nisi, in efficitur est facilisis non. Etiam aliquet molestie rhoncus. Sed ac tortor et orci consequat pulvinar nec eu turpis. Curabitur consectetur mauris eget mi dictum congue. Praesent imperdiet quam nisl, a consequat metus consequat vel. Suspendisse vel dolor tristique, fringilla erat vel, bibendum nulla.
    
    Donec rhoncus congue venenatis. Maecenas ultrices dui nec metus tempus luctus. Nullam venenatis odio ut finibus aliquam. Duis auctor nec tortor sed ornare. Duis vitae feugiat risus. Cras eu est a dolor pretium lacinia in nec dolor. Donec vel fringilla dui. Nullam urna libero, accumsan non enim nec, eleifend luctus diam. Sed at placerat libero, vestibulum ultricies elit. Nullam aliquet justo tristique, faucibus mauris quis, pretium turpis. Suspendisse vel ante tristique, dignissim tellus vitae, euismod ex. Donec iaculis bibendum urna aliquet maximus. Fusce at sem auctor, facilisis augue quis, posuere ipsum. Phasellus eu urna fermentum, eleifend lorem in, feugiat ipsum. Nulla facilisi. Mauris aliquet varius nisl, vel cursus mi sagittis ut.
    
    Fusce placerat lorem vel elit iaculis, at pellentesque sapien lobortis. Duis ante lorem, maximus sit amet ornare vitae, congue sit amet lacus. Fusce blandit congue velit eu porttitor. Praesent lacus felis, pulvinar non vestibulum at, ultricies eu massa. Suspendisse eros enim, fringilla et nibh ac, mattis sagittis dolor. Nulla ullamcorper lacus id sapien tincidunt, vel efficitur metus dapibus. Pellentesque rhoncus, lacus in molestie elementum, enim quam rhoncus tortor, sed varius turpis tellus vitae augue. Nam a urna mauris. Sed mi mauris, auctor vitae leo ut, tincidunt rhoncus turpis. Mauris sem lacus, mollis non commodo eu, eleifend a eros. Quisque vestibulum aliquam placerat. Cras tincidunt quam sed iaculis volutpat. Nunc metus enim.".into())));
}

fn criterion_benchmark2(c: &mut Criterion) {
    c.bench_function("regex", |b| {
        // b.iter(|| func2("aaaaaaaaaaaaaaaaaaab".into(), "a*a*a*a*a*a*a*a*a*a*".into()))
        b.iter(|| func2("aab".into(), "a*a+".into()))
    });
}

criterion_group!(benches, criterion_benchmark2);
criterion_main!(benches);
